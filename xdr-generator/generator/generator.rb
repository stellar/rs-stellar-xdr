require 'xdrgen'

class Generator < Xdrgen::Generators::Base

  AST = Xdrgen::AST

  def generate
    @already_rendered = []
    path = "#{@namespace}.rs"
    out = @output.open(path)

    @types = build_type_list(@top)
    @type_field_types = build_type_field_types(@top)
    @type_definitions = build_type_definitions_map(@top)
    @sparse_types = Set.new  # Track all sparse type names globally

    render_top_matter(out)
    render_lib(out)
    render_definitions(out, @top)
    render_sparse_types(out)
    # Add sparse types to the types list
    @types.merge(@sparse_types)
    render_enum_of_all_types(out, @types)
    out.break
  end

  private

  def build_type_list(node)
    types = Set.new
    ingest_node = lambda do |n|
      case n
      when AST::Definitions::Struct, AST::Definitions::Enum, AST::Definitions::Union, AST::Definitions::Typedef
        types << name(n)
      end
      n.definitions.each{ |nn| ingest_node.call(nn) } if n.respond_to?(:definitions)
      n.nested_definitions.each{ |nn| ingest_node.call(nn) } if n.respond_to?(:nested_definitions)
    end
    ingest_node.call(node)
    types
  end

  def build_type_field_types(node)
    types = Hash.new { |h, k| h[k] = [] }
    ingest_node = lambda do |n|
      n.definitions.each{ |nn| ingest_node.call(nn) } if n.respond_to?(:definitions)
      n.nested_definitions.each{ |nn| ingest_node.call(nn) } if n.respond_to?(:nested_definitions)
      case n
      when AST::Definitions::Struct
        n.members.each do |m|
          types[name(n)] << base_reference(m.declaration.type)
        end
      when AST::Definitions::Union ;
        union_cases(n) do |_, arm|
          types[name(n)] << base_reference(arm.type) unless arm.void?
        end
      end
    end
    ingest_node.call(node)
    types
  end

  # Determines if 'type' is referenced directly or indirectly by 'type_with_fields'.
  # Used to determine if 'type_with_fields' has a recursive relationship to 'type'.
  def is_type_in_type_field_types(type_with_fields, type, seen = [])
    return false if seen.include?(type_with_fields)
    seen << type_with_fields
    @type_field_types[type_with_fields].any? do |field_type|
      if field_type == type
        true
      else
        is_type_in_type_field_types(field_type, type, seen)
      end
    end
  end

  # Build a map of type name -> definition for quick lookup
  def build_type_definitions_map(node)
    definitions = {}
    ingest_node = lambda do |n|
      case n
      when AST::Definitions::Struct, AST::Definitions::Enum, AST::Definitions::Union, AST::Definitions::Typedef
        definitions[name(n)] = n
      end
      n.definitions.each{ |nn| ingest_node.call(nn) } if n.respond_to?(:definitions)
      n.nested_definitions.each{ |nn| ingest_node.call(nn) } if n.respond_to?(:nested_definitions)
    end
    ingest_node.call(node)
    definitions
  end

  def render_top_matter(out)
    out.puts <<-EOS.strip_heredoc
      // Module #{@namepsace} is generated from:
      //  #{@output.relative_source_paths.join("\n//  ")}
    EOS
    out.break
    out.puts "#![allow(unknown_lints, clippy::missing_errors_doc, clippy::unreadable_literal, clippy::semicolon_if_nothing_returned, clippy::absurd_extreme_comparisons, clippy::derivable_impls, clippy::cast_possible_wrap, clippy::len_zero, clippy::unnecessary_semicolon)]"
    out.break
    source_paths_sha256_hashes = @output.relative_source_path_sha256_hashes
    out.puts <<-EOS.strip_heredoc
      /// `XDR_FILES_SHA256` is a list of pairs of source files and their SHA256 hashes.
      pub const XDR_FILES_SHA256: [(&str, &str); #{source_paths_sha256_hashes.count}] = [
        #{source_paths_sha256_hashes.map(){ |path, hash| %{("#{path}", "#{hash}")} }.join(",\n")}
      ];
    EOS
    out.break
  end

  def render_lib(out)
    header = IO.read(__dir__ + "/header.rs")
    out.puts(header)
    out.break
  end

  def render_enum_of_all_types(out, types)
    # Separate sparse types from regular types for methods that don't support sparse
    non_sparse_types = types.reject { |t| @sparse_types.include?(t) }

    out.puts <<-EOS.strip_heredoc
    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    #[cfg_attr(
      all(feature = "serde", feature = "alloc"),
      derive(serde::Serialize, serde::Deserialize),
      serde(rename_all = "snake_case")
    )]
    #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
    #[allow(non_camel_case_types)]
    pub enum TypeVariant {
        #{types.map { |t| "#{t}," }.join("\n")}
    }

    impl TypeVariant {
        pub const VARIANTS: [TypeVariant; #{types.count}] = [ #{types.map { |t| "TypeVariant::#{t}," }.join("\n")} ];
        pub const VARIANTS_STR: [&'static str; #{types.count}] = [ #{types.map { |t| "\"#{t}\"," }.join("\n")} ];

        #[must_use]
        #[allow(clippy::too_many_lines)]
        pub const fn name(&self) -> &'static str {
            match self {
                #{types.map { |t| "Self::#{t} => \"#{t}\"," }.join("\n")}
            }
        }

        #[must_use]
        #[allow(clippy::too_many_lines)]
        pub const fn variants() -> [TypeVariant; #{types.count}] {
            Self::VARIANTS
        }

        #[cfg(feature = "schemars")]
        #[must_use]
        #[allow(clippy::too_many_lines)]
        pub fn json_schema(&self, gen: schemars::gen::SchemaGenerator) -> schemars::schema::RootSchema {
            match self {
                #{types.map { |t| "Self::#{t} => gen.into_root_schema_for::<#{t}>()," }.join("\n")}
            }
        }
    }

    impl Name for TypeVariant {
        #[must_use]
        fn name(&self) -> &'static str {
            Self::name(self)
        }
    }

    impl Variants<TypeVariant> for TypeVariant {
        fn variants() -> slice::Iter<'static, TypeVariant> {
            Self::VARIANTS.iter()
        }
    }

    impl core::str::FromStr for TypeVariant {
        type Err = Error;
        #[allow(clippy::too_many_lines)]
        fn from_str(s: &str) -> Result<Self, Error> {
            match s {
                #{types.map { |t| "\"#{t}\" => Ok(Self::#{t})," }.join("\n")}
                _ => Err(Error::Invalid),
            }
        }
    }

    #[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    #[cfg_attr(
      all(feature = "serde", feature = "alloc"),
      derive(serde::Serialize, serde::Deserialize),
      serde(rename_all = "snake_case"),
      serde(untagged),
    )]
    #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
    #[allow(non_camel_case_types)]
    pub enum Type {
        #{types.map { |t| "#{t}(Box<#{t}>)," }.join("\n")}
    }

    impl Type {
        pub const VARIANTS: [TypeVariant; #{types.count}] = [ #{types.map { |t| "TypeVariant::#{t}," }.join("\n")} ];
        pub const VARIANTS_STR: [&'static str; #{types.count}] = [ #{types.map { |t| "\"#{t}\"," }.join("\n")} ];

        #[cfg(feature = "std")]
        #[allow(clippy::too_many_lines)]
        pub fn read_xdr<R: Read>(v: TypeVariant, r: &mut Limited<R>) -> Result<Self, Error> {
            match v {
                #{types.map { |t| "TypeVariant::#{t} => r.with_limited_depth(|r| Ok(Self::#{t}(Box::new(#{t}::read_xdr(r)?))))," }.join("\n")}
            }
        }

        #[cfg(feature = "base64")]
        pub fn read_xdr_base64<R: Read>(v: TypeVariant, r: &mut Limited<R>) -> Result<Self, Error> {
            let mut dec = Limited::new(
                base64::read::DecoderReader::new(
                    SkipWhitespace::new(&mut r.inner),
                    &base64::engine::general_purpose::STANDARD,
                ),
                r.limits.clone(),
            );
            let t = Self::read_xdr(v, &mut dec)?;
            Ok(t)
        }

        #[cfg(feature = "std")]
        pub fn read_xdr_to_end<R: Read>(v: TypeVariant, r: &mut Limited<R>) -> Result<Self, Error> {
            let s = Self::read_xdr(v, r)?;
            // Check that any further reads, such as this read of one byte, read no
            // data, indicating EOF. If a byte is read the data is invalid.
            if r.read(&mut [0u8; 1])? == 0 {
                Ok(s)
            } else {
                Err(Error::Invalid)
            }
        }

        #[cfg(feature = "base64")]
        pub fn read_xdr_base64_to_end<R: Read>(v: TypeVariant, r: &mut Limited<R>) -> Result<Self, Error> {
            let mut dec = Limited::new(
                base64::read::DecoderReader::new(
                    SkipWhitespace::new(&mut r.inner),
                    &base64::engine::general_purpose::STANDARD,
                ),
                r.limits.clone(),
            );
            let t = Self::read_xdr_to_end(v, &mut dec)?;
            Ok(t)
        }

        #[cfg(feature = "std")]
        #[allow(clippy::too_many_lines)]
        pub fn read_xdr_iter<R: Read>(v: TypeVariant, r: &mut Limited<R>) -> Box<dyn Iterator<Item=Result<Self, Error>> + '_> {
            match v {
                #{types.map { |t| "TypeVariant::#{t} => Box::new(ReadXdrIter::<_, #{t}>::new(&mut r.inner, r.limits.clone()).map(|r| r.map(|t| Self::#{t}(Box::new(t)))))," }.join("\n")}
            }
        }

        #[cfg(feature = "std")]
        #[allow(clippy::too_many_lines)]
        pub fn read_xdr_framed_iter<R: Read>(v: TypeVariant, r: &mut Limited<R>) -> Box<dyn Iterator<Item=Result<Self, Error>> + '_> {
            match v {
                #{non_sparse_types.map { |t| "TypeVariant::#{t} => Box::new(ReadXdrIter::<_, Frame<#{t}>>::new(&mut r.inner, r.limits.clone()).map(|r| r.map(|t| Self::#{t}(Box::new(t.0)))))," }.join("\n")}
                #{@sparse_types.empty? ? "" : "_ => Box::new(core::iter::once(Err(Error::Invalid))),"}
            }
        }

        #[cfg(feature = "base64")]
        #[allow(clippy::too_many_lines)]
        pub fn read_xdr_base64_iter<R: Read>(v: TypeVariant, r: &mut Limited<R>) -> Box<dyn Iterator<Item=Result<Self, Error>> + '_> {
            let dec = base64::read::DecoderReader::new(
                SkipWhitespace::new(&mut r.inner),
                &base64::engine::general_purpose::STANDARD,
            );
            match v {
                #{types.map { |t| "TypeVariant::#{t} => Box::new(ReadXdrIter::<_, #{t}>::new(dec, r.limits.clone()).map(|r| r.map(|t| Self::#{t}(Box::new(t)))))," }.join("\n")}
            }
        }

        #[cfg(feature = "std")]
        pub fn from_xdr<B: AsRef<[u8]>>(v: TypeVariant, bytes: B, limits: Limits) -> Result<Self, Error> {
            let mut cursor = Limited::new(Cursor::new(bytes.as_ref()), limits);
            let t = Self::read_xdr_to_end(v, &mut cursor)?;
            Ok(t)
        }

        #[cfg(feature = "base64")]
        pub fn from_xdr_base64(v: TypeVariant, b64: impl AsRef<[u8]>, limits: Limits) -> Result<Self, Error> {
            let mut dec = Limited::new(
                base64::read::DecoderReader::new(
                    SkipWhitespace::new(Cursor::new(b64)),
                    &base64::engine::general_purpose::STANDARD,
                ),
                limits,
            );
            let t = Self::read_xdr_to_end(v, &mut dec)?;
            Ok(t)
        }

        #[cfg(all(feature = "std", feature = "serde_json"))]
        #[deprecated(note = "use from_json")]
        pub fn read_json(v: TypeVariant, r: impl Read) -> Result<Self, Error> {
            Self::from_json(v, r)
        }

        #[cfg(all(feature = "std", feature = "serde_json"))]
        #[allow(clippy::too_many_lines)]
        pub fn from_json(v: TypeVariant, r: impl Read) -> Result<Self, Error> {
            match v {
                #{types.map { |t| "TypeVariant::#{t} => Ok(Self::#{t}(Box::new(serde_json::from_reader(r)?)))," }.join("\n")}
            }
        }

        #[cfg(all(feature = "std", feature = "serde_json"))]
        #[allow(clippy::too_many_lines)]
        pub fn deserialize_json<'r, R: serde_json::de::Read<'r>>(v: TypeVariant, r: &mut serde_json::de::Deserializer<R>) -> Result<Self, Error> {
            match v {
                #{types.map { |t| "TypeVariant::#{t} => Ok(Self::#{t}(Box::new(serde::de::Deserialize::deserialize(r)?)))," }.join("\n")}
            }
        }

        #[cfg(feature = "arbitrary")]
        #[allow(clippy::too_many_lines)]
        pub fn arbitrary(v: TypeVariant, u: &mut arbitrary::Unstructured<'_>) -> Result<Self, Error> {
            match v {
                #{types.map { |t| "TypeVariant::#{t} => Ok(Self::#{t}(Box::new(#{t}::arbitrary(u)?)))," }.join("\n")}
            }
        }

        #[cfg(feature = "alloc")]
        #[must_use]
        #[allow(clippy::too_many_lines, clippy::missing_panics_doc)]
        pub fn default(v: TypeVariant) -> Self {
            match v {
                #{non_sparse_types.map { |t| "TypeVariant::#{t} => Self::#{t}(Box::default())," }.join("\n")}
                #{@sparse_types.empty? ? "" : "_ => panic!(\"sparse types do not support default\"),"}
            }
        }

        #[cfg(feature = "alloc")]
        #[must_use]
        #[allow(clippy::too_many_lines)]
        pub fn value(&self) -> &dyn core::any::Any {
            #[allow(clippy::match_same_arms)]
            match self {
                #{types.map { |t| "Self::#{t}(ref v) => v.as_ref()," }.join("\n")}
            }
        }

        #[must_use]
        #[allow(clippy::too_many_lines)]
        pub const fn name(&self) -> &'static str {
            match self {
                #{types.map { |t| "Self::#{t}(_) => \"#{t}\"," }.join("\n")}
            }
        }

        #[must_use]
        #[allow(clippy::too_many_lines)]
        pub const fn variants() -> [TypeVariant; #{types.count}] {
            Self::VARIANTS
        }

        #[must_use]
        #[allow(clippy::too_many_lines)]
        pub const fn variant(&self) -> TypeVariant {
            match self {
                #{types.map { |t| "Self::#{t}(_) => TypeVariant::#{t}," }.join("\n")}
            }
        }
    }

    impl Name for Type {
        #[must_use]
        fn name(&self) -> &'static str {
            Self::name(self)
        }
    }

    impl Variants<TypeVariant> for Type {
        fn variants() -> slice::Iter<'static, TypeVariant> {
            Self::VARIANTS.iter()
        }
    }

    impl WriteXdr for Type {
        #[cfg(feature = "std")]
        #[allow(clippy::too_many_lines)]
        fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
            match self {
                #{non_sparse_types.map { |t| "Self::#{t}(v) => v.write_xdr(w)," }.join("\n")}
                #{@sparse_types.empty? ? "" : "_ => Err(Error::Invalid),"}
            }
        }
    }
    EOS
    out.break
  end

  def render_definitions(out, node)
    node.definitions.each{|n| render_definition out, n }
    node.namespaces.each{|n| render_definitions out, n }
  end

  def render_definition(out, defn)
    if @already_rendered.include? name(defn)

      unless defn.is_a?(AST::Definitions::Namespace)
        $stderr.puts "warn: #{name(defn)} is defined twice.  skipping"
      end

      return
    end

    render_nested_definitions(out, defn)
    render_source_comment(out, defn)

    @already_rendered << name(defn)

    case defn
    when AST::Definitions::Struct ;
      render_struct out, defn
    when AST::Definitions::Enum ;
      render_enum out, defn
    when AST::Definitions::Union ;
      render_union out, defn
    when AST::Definitions::Typedef ;
      render_typedef out, defn
    when AST::Definitions::Const ;
      render_const out, defn
    end
  end

  def render_nested_definitions(out, defn)
    return unless defn.respond_to? :nested_definitions
    defn.nested_definitions.each{|ndefn| render_definition out, ndefn}
  end

  def render_source_comment(out, defn)
    return if defn.is_a?(AST::Definitions::Namespace)

    out.puts <<-EOS.strip_heredoc
      /// #{name defn} is an XDR #{defn.class.name.demodulize} defined as:
      ///
      /// ```text
    EOS

    out.puts "/// " + defn.text_value.split("\n").join("\n/// ")

    out.puts <<-EOS.strip_heredoc
      /// ```
      ///
    EOS
  end

  def render_struct(out, struct)
    out.puts %{#[cfg_attr(feature = "alloc", derive(Default))]} if !@options[:custom_default_impl].include?(name struct)
    out.puts "#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]"
    out.puts %{#[cfg_eval::cfg_eval]}
    out.puts %{#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]}
    if @options[:custom_str_impl].include?(name struct)
      out.puts %{#[cfg_attr(all(feature = "serde", feature = "alloc"), derive(serde_with::SerializeDisplay))]}
    else
      out.puts %{#[cfg_attr(all(feature = "serde", feature = "alloc"), serde_with::serde_as, derive(serde::Serialize, serde::Deserialize), serde(rename_all = "snake_case"))]}
    end
    if !@options[:custom_str_impl].include?(name struct)
      out.puts %{#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]}
    end
    out.puts "pub struct #{name struct} {"
    out.indent do
      struct.members.each do |m|
        out.puts_if(field_attrs(struct, m.declaration.type)) if !@options[:custom_str_impl].include?(name struct)
        out.puts "pub #{field_name m}: #{reference(struct, m.declaration.type)},"
      end
    end
    out.puts "}"
    out.puts ""
    out.puts <<-EOS.strip_heredoc
    impl ReadXdr for #{name struct} {
        #[cfg(feature = "std")]
        fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
            r.with_limited_depth(|r| {
                Ok(Self{
                  #{struct.members.map do |m|
                    "#{field_name(m)}: #{reference_to_call(struct, m.declaration.type)}::read_xdr(r)?,"
                  end.join("\n")}
                })
            })
        }
    }

    impl WriteXdr for #{name struct} {
        #[cfg(feature = "std")]
        fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
            w.with_limited_depth(|w| {
                #{struct.members.map do |m|
                  "self.#{field_name(m)}.write_xdr(w)?;"
                end.join("\n")}
                Ok(())
            })
        }
    }

    impl SkipXdr for #{name struct} {
        #[cfg(feature = "std")]
        fn skip_xdr<R: Read>(r: &mut Limited<R>) -> Result<(), Error> {
            r.with_limited_depth(|r| {
                #{struct.members.map do |m|
                  "#{reference_to_call(struct, m.declaration.type)}::skip_xdr(r)?;"
                end.join("\n")}
                Ok(())
            })
        }
    }

    impl SeekableSkipXdr for #{name struct} {
        #[cfg(feature = "std")]
        fn seekable_skip_xdr<R: Read + Seek>(r: &mut Limited<R>) -> Result<(), Error> {
            r.with_limited_depth(|r| {
                #{struct.members.map do |m|
                  "#{reference_to_call(struct, m.declaration.type)}::seekable_skip_xdr(r)?;"
                end.join("\n")}
                Ok(())
            })
        }
    }
    EOS
    # Include a deserializer that will deserialize via the FromStr
    # implementation, but also deserialize the original struct, present in
    # the JSON as a map. The reason for the second option for
    # deserialization is that types that we're adding string
    # representations for were previously deserializable via a map to their
    # struct, and so this improves the backwards compatibility.
    # Note that this is only done for structs and not other types (typedef,
    # enum, union), because struct is the only type that maps to JSON in a
    # way that is unambiguous with a secondary form in string type.
    out.puts <<-EOS.strip_heredoc if @options[:custom_str_impl].include?(name struct)
    #[cfg(all(feature = "serde", feature = "alloc"))]
    impl<'de> serde::Deserialize<'de> for #{name struct} {
        fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error> where D: serde::Deserializer<'de> {
            use serde::Deserialize;
            #[derive(Deserialize)]
            struct #{name struct} {
                #{struct.members.map do |m|
                  "#{field_name(m)}: #{reference(struct, m.declaration.type)},"
                end.join("\n")}
            }
            #[derive(Deserialize)]
            #[serde(untagged)]
            enum #{name struct}OrString<'a> {
                Str(&'a str),
                String(String),
                #{name struct}(#{name struct}),
            }
            match #{name struct}OrString::deserialize(deserializer)? {
                #{name struct}OrString::Str(s) => s.parse().map_err(serde::de::Error::custom),
                #{name struct}OrString::String(s) => s.parse().map_err(serde::de::Error::custom),
                #{name struct}OrString::#{name struct}(#{name struct} {
                    #{struct.members.map do |m| "#{field_name(m)}," end.join(" ")}
                }) => Ok(self::#{name struct} {
                    #{struct.members.map do |m| "#{field_name(m)}," end.join(" ")}
                }),
            }
        }
    }
    EOS
    out.break
  end

  def render_enum(out, enum)
    out.puts "// enum"
    out.puts %{#[cfg_attr(feature = "alloc", derive(Default))]} if !@options[:custom_default_impl].include?(name enum)
    out.puts "#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]"
    out.puts %{#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]}
    if @options[:custom_str_impl].include?(name enum)
      out.puts %{#[cfg_attr(all(feature = "serde", feature = "alloc"), derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr))]}
    else
      out.puts %{#[cfg_attr(all(feature = "serde", feature = "alloc"), derive(serde::Serialize, serde::Deserialize), serde(rename_all = "snake_case"))]}
    end
    if !@options[:custom_str_impl].include?(name enum)
      out.puts %{#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]}
    end
    out.puts "#[repr(i32)]"
    out.puts "pub enum #{name enum} {"
    out.indent do
      enum.members.each_with_index do |m, i|
        out.puts(%{#[cfg_attr(feature = "alloc", default)]}) if i == 0
        out.puts "#{name m} = #{m.value},"
      end
    end
    out.puts '}'
    out.puts ""
    out.puts <<-EOS.strip_heredoc
    impl #{name enum} {
        pub const VARIANTS: [#{name enum}; #{enum.members.count}] = [ #{enum.members.map { |m| "#{name enum}::#{name m}," }.join("\n")} ];
        pub const VARIANTS_STR: [&'static str; #{enum.members.count}] = [ #{enum.members.map { |m| "\"#{name m}\"," }.join("\n")} ];

        #[must_use]
        pub const fn name(&self) -> &'static str {
            match self {
                #{enum.members.map do |m|
                  "Self::#{name m} => \"#{name m}\","
                end.join("\n")}
            }
        }

        #[must_use]
        pub const fn variants() -> [#{name enum}; #{enum.members.count}] {
            Self::VARIANTS
        }
    }

    impl Name for #{name enum} {
        #[must_use]
        fn name(&self) -> &'static str {
            Self::name(self)
        }
    }

    impl Variants<#{name enum}> for #{name enum} {
        fn variants() -> slice::Iter<'static, #{name enum}> {
            Self::VARIANTS.iter()
        }
    }

    impl Enum for #{name enum} {}

    impl fmt::Display for #{name enum} {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(self.name())
        }
    }

    impl TryFrom<i32> for #{name enum} {
        type Error = Error;

        fn try_from(i: i32) -> Result<Self, Error> {
            let e = match i {
                #{enum.members.map do |m| "#{m.value} => #{name enum}::#{name m}," end.join("\n")}
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(e)
        }
    }

    impl From<#{name enum}> for i32 {
        #[must_use]
        fn from(e: #{name enum}) -> Self {
            e as Self
        }
    }

    impl ReadXdr for #{name enum} {
        #[cfg(feature = "std")]
        fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
            r.with_limited_depth(|r| {
                let e = i32::read_xdr(r)?;
                let v: Self = e.try_into()?;
                Ok(v)
            })
        }
    }

    impl WriteXdr for #{name enum} {
        #[cfg(feature = "std")]
        fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
            w.with_limited_depth(|w| {
                let i: i32 = (*self).into();
                i.write_xdr(w)
            })
        }
    }

    impl SkipXdr for #{name enum} {
        #[cfg(feature = "std")]
        fn skip_xdr<R: Read>(r: &mut Limited<R>) -> Result<(), Error> {
            r.with_limited_depth(|r| {
                i32::skip_xdr(r)
            })
        }
    }

    impl SeekableSkipXdr for #{name enum} {
        #[cfg(feature = "std")]
        fn seekable_skip_xdr<R: Read + Seek>(r: &mut Limited<R>) -> Result<(), Error> {
            r.with_limited_depth(|r| {
                i32::seekable_skip_xdr(r)
            })
        }
    }
    EOS
    out.break
  end

  def union_is_idents(union)
    union.normal_arms.first&.cases.first&.value.is_a?(AST::Identifier)
  end

  def union_cases(union)
    results = []
    union.normal_arms.each do |arm|
      arm.cases.each do |kase|
          if kase.value.is_a?(AST::Identifier)
            case_name = kase.name_short.underscore.camelize
            value = nil
          else
            case_name = "V#{kase.value.value}"
            value = kase.value.value
          end
          results << yield(case_name, arm, value)
      end
    end
    results
  end

  def render_union(out, union)
    if union.default_arm.present?
      $stderr.puts "warn: union #{name union} includes default arms and default arms are not supported in the rust generator"
    end
    discriminant_type = reference(nil, union.discriminant.type)
    discriminant_type_builtin = is_builtin_type(union.discriminant.type) || (is_builtin_type(union.discriminant.type.resolved_type.type) if union.discriminant.type.respond_to?(:resolved_type) && AST::Definitions::Typedef === union.discriminant.type.resolved_type)
    out.puts "// union with discriminant #{discriminant_type}"
    out.puts %{#[cfg_eval::cfg_eval]}
    out.puts "#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]"
    out.puts %{#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]}
    if @options[:custom_str_impl].include?(name union)
      out.puts %{#[cfg_attr(all(feature = "serde", feature = "alloc"), derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr))]}
    else
      out.puts %{#[cfg_attr(all(feature = "serde", feature = "alloc"), serde_with::serde_as, derive(serde::Serialize, serde::Deserialize), serde(rename_all = "snake_case"))]}
    end
    if !@options[:custom_str_impl].include?(name union)
      out.puts %{#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]}
    end
    out.puts "#[allow(clippy::large_enum_variant)]"
    out.puts "pub enum #{name union} {"
    union_case_count = 0
    out.indent do
      union_cases(union) do |case_name, arm|
        union_case_count += 1
        if arm.void?
          out.puts "#{case_name}#{"(())" unless arm.void?},"
        else
          out.puts "#{case_name}("
          out.puts_if(field_attrs(union, arm.type)) if !@options[:custom_str_impl].include?(name union)
          out.puts "  #{reference(union, arm.type)}"
          out.puts "),"
        end
      end
    end
    out.puts '}'
    out.puts ""
    if !@options[:custom_default_impl].include?(name union)
      union_cases(union) do |case_name, arm|
        out.puts <<-EOS.strip_heredoc
        #[cfg(feature = "alloc")]
        impl Default for #{name union} {
            fn default() -> Self {
                Self::#{case_name}#{"(#{reference_to_call(union, arm.type)}::default())" if !arm.void?}
            }
        }
        EOS
        break # output the above for the first union case
      end
      out.puts ""
    end
    out.puts <<-EOS.strip_heredoc
    impl #{name union} {
        pub const VARIANTS: [#{discriminant_type}; #{union_case_count}] = [
            #{union_cases(union) do |case_name, arm, value|
              value.nil?                ? "#{discriminant_type}::#{case_name}," :
              discriminant_type_builtin ? "#{value}," :
                                          "#{discriminant_type}(#{value}),"
            end.join("\n")}
        ];
        pub const VARIANTS_STR: [&'static str; #{union_case_count}] = [
            #{union_cases(union) do |case_name, arm, value|
              "\"#{case_name}\","
            end.join("\n")}
        ];

        #[must_use]
        pub const fn name(&self) -> &'static str {
            match self {
                #{union_cases(union) do |case_name, arm|
                  "Self::#{case_name}#{"(_)" unless arm.void?} => \"#{case_name}\","
                end.join("\n")}
            }
        }

        #[must_use]
        pub const fn discriminant(&self) -> #{discriminant_type} {
            #[allow(clippy::match_same_arms)]
            match self {
                #{union_cases(union) do |case_name, arm, value|
                  "Self::#{case_name}#{"(_)" unless arm.void?} => #{
                    value.nil?                ? "#{discriminant_type}::#{case_name}" :
                    discriminant_type_builtin ? "#{value}" :
                                                "#{discriminant_type}(#{value})"
                  },"
                end.join("\n")}
            }
        }

        #[must_use]
        pub const fn variants() -> [#{discriminant_type}; #{union_case_count}] {
            Self::VARIANTS
        }
    }

    impl Name for #{name union} {
        #[must_use]
        fn name(&self) -> &'static str {
            Self::name(self)
        }
    }

    impl Discriminant<#{discriminant_type}> for #{name union} {
        #[must_use]
        fn discriminant(&self) -> #{discriminant_type} {
            Self::discriminant(self)
        }
    }

    impl Variants<#{discriminant_type}> for #{name union} {
        fn variants() -> slice::Iter<'static, #{discriminant_type}> {
            Self::VARIANTS.iter()
        }
    }

    impl Union<#{discriminant_type}> for #{name union} {}

    impl ReadXdr for #{name union} {
        #[cfg(feature = "std")]
        fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
            r.with_limited_depth(|r| {
                let dv: #{discriminant_type} = <#{discriminant_type} as ReadXdr>::read_xdr(r)?;
                #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
                let v = match dv {
                    #{union_cases(union) do |case_name, arm, value|
                      "#{
                        value.nil? ? "#{discriminant_type}::#{case_name}" : "#{value}"
                      } => #{
                        arm.void? ? "Self::#{case_name}" : "Self::#{case_name}(#{reference_to_call(union, arm.type)}::read_xdr(r)?)"
                      },"
                    end.join("\n")}
                    #[allow(unreachable_patterns)]
                    _ => return Err(Error::Invalid),
                };
                Ok(v)
            })
        }
    }

    impl WriteXdr for #{name union} {
        #[cfg(feature = "std")]
        fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
            w.with_limited_depth(|w| {
                self.discriminant().write_xdr(w)?;
                #[allow(clippy::match_same_arms)]
                match self {
                    #{union_cases(union) do |case_name, arm, value|
                      if arm.void?
                        "Self::#{case_name} => ().write_xdr(w)?,"
                      else
                        "Self::#{case_name}(v) => v.write_xdr(w)?,"
                      end
                    end.join("\n")}
                };
                Ok(())
            })
        }
    }

    impl SkipXdr for #{name union} {
        #[cfg(feature = "std")]
        fn skip_xdr<R: Read>(r: &mut Limited<R>) -> Result<(), Error> {
            r.with_limited_depth(|r| {
                let dv: #{discriminant_type} = <#{discriminant_type} as ReadXdr>::read_xdr(r)?;
                #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
                match dv {
                    #{union_cases(union) do |case_name, arm, value|
                      "#{
                        value.nil? ? "#{discriminant_type}::#{case_name}" : "#{value}"
                      } => #{
                        arm.void? ? "()" : "#{reference_to_call(union, arm.type)}::skip_xdr(r)?"
                      },"
                    end.join("\n")}
                    #[allow(unreachable_patterns)]
                    _ => return Err(Error::Invalid),
                };
                Ok(())
            })
        }
    }

    impl SeekableSkipXdr for #{name union} {
        #[cfg(feature = "std")]
        fn seekable_skip_xdr<R: Read + Seek>(r: &mut Limited<R>) -> Result<(), Error> {
            r.with_limited_depth(|r| {
                let dv: #{discriminant_type} = <#{discriminant_type} as ReadXdr>::read_xdr(r)?;
                #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
                match dv {
                    #{union_cases(union) do |case_name, arm, value|
                      "#{
                        value.nil? ? "#{discriminant_type}::#{case_name}" : "#{value}"
                      } => #{
                        arm.void? ? "()" : "#{reference_to_call(union, arm.type)}::seekable_skip_xdr(r)?"
                      },"
                    end.join("\n")}
                    #[allow(unreachable_patterns)]
                    _ => return Err(Error::Invalid),
                };
                Ok(())
            })
        }
    }
    EOS
    out.break
  end

  def render_typedef(out, typedef)
    if is_builtin_type(typedef.type)
      out.puts "pub type #{name typedef} = #{reference(typedef, typedef.type)};"
    else
      out.puts %{#[cfg_eval::cfg_eval]}
      if !@options[:custom_default_impl].include?(name typedef)
        if is_var_array_type(typedef.type)
          out.puts "#[derive(Default)]"
        else
          out.puts %{#[cfg_attr(feature = "alloc", derive(Default))]}
        end
      end
      out.puts "#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]"
      out.puts %{#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]}
      if is_fixed_array_opaque(typedef.type) || @options[:custom_str_impl].include?(name typedef)
        out.puts %{#[cfg_attr(all(feature = "serde", feature = "alloc"), derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr))]}
      else
        out.puts %{#[cfg_attr(all(feature = "serde", feature = "alloc"), serde_with::serde_as, derive(serde::Serialize, serde::Deserialize), serde(rename_all = "snake_case"))]}
      end
      if !is_fixed_array_opaque(typedef.type) && !@options[:custom_str_impl].include?(name typedef)
        out.puts %{#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]}
      end
      if !is_fixed_array_opaque(typedef.type)
        out.puts "#[derive(Debug)]"
      end
      out.puts "pub struct #{name typedef}("
      out.puts_if(field_attrs(typedef, typedef.type)) if !@options[:custom_str_impl].include?(name typedef)
      out.puts "  pub #{reference(typedef, typedef.type)}"
      out.puts ");"
      out.puts ""
      if is_fixed_array_opaque(typedef.type)
      out.puts <<-EOS.strip_heredoc
      impl core::fmt::Debug for #{name typedef} {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let v = &self.0;
            write!(f, "#{name typedef}(")?;
            for b in v {
                write!(f, "{b:02x}")?;
            }
            write!(f, ")")?;
            Ok(())
        }
      }
      EOS
      end
      if is_fixed_array_opaque(typedef.type) && !@options[:custom_str_impl].include?(name typedef)
      out.puts <<-EOS.strip_heredoc
      impl core::fmt::Display for #{name typedef} {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let v = &self.0;
            for b in v {
                write!(f, "{b:02x}")?;
            }
            Ok(())
        }
      }

      #[cfg(feature = "alloc")]
      impl core::str::FromStr for #{name typedef} {
        type Err = Error;
        fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
            hex::decode(s).map_err(|_| Error::InvalidHex)?.try_into()
        }
      }
      EOS
      end
      if is_fixed_array_opaque(typedef.type) && !@options[:custom_str_impl].include?(name typedef)
      out.puts <<-EOS.strip_heredoc
      #[cfg(feature = "schemars")]
      impl schemars::JsonSchema for #{name typedef} {
          fn schema_name() -> String {
              "#{name typedef}".to_string()
          }

          fn is_referenceable() -> bool {
              false
          }

          fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
              let schema = String::json_schema(gen);
              if let schemars::schema::Schema::Object(mut schema) = schema {
                  schema.extensions.insert(
                      "contentEncoding".to_owned(),
                      serde_json::Value::String("hex".to_string()),
                  );
                  schema.extensions.insert(
                      "contentMediaType".to_owned(),
                      serde_json::Value::String("application/binary".to_string()),
                  );
                  let string = *schema.string.unwrap_or_default().clone();
                  schema.string = Some(Box::new(schemars::schema::StringValidation {
                      max_length: #{typedef.type.size}_u32.checked_mul(2).map(Some).unwrap_or_default(),
                      min_length: #{typedef.type.size}_u32.checked_mul(2).map(Some).unwrap_or_default(),
                      ..string
                  }));
                  schema.into()
              } else {
                  schema
              }
          }
      }
      EOS
      end
      out.puts <<-EOS.strip_heredoc
      impl From<#{name typedef}> for #{reference(typedef, typedef.type)} {
          #[must_use]
          fn from(x: #{name typedef}) -> Self {
              x.0
          }
      }

      impl From<#{reference(typedef, typedef.type)}> for #{name typedef} {
          #[must_use]
          fn from(x: #{reference(typedef, typedef.type)}) -> Self {
              #{name typedef}(x)
          }
      }

      impl AsRef<#{reference(typedef, typedef.type)}> for #{name typedef} {
          #[must_use]
          fn as_ref(&self) -> &#{reference(typedef, typedef.type)} {
              &self.0
          }
      }

      impl ReadXdr for #{name typedef} {
          #[cfg(feature = "std")]
          fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
              r.with_limited_depth(|r| {
                  let i = #{reference_to_call(typedef, typedef.type)}::read_xdr(r)?;
                  let v = #{name typedef}(i);
                  Ok(v)
              })
          }
      }

      impl WriteXdr for #{name typedef} {
          #[cfg(feature = "std")]
          fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
              w.with_limited_depth(|w|{ self.0.write_xdr(w) })
          }
      }

      impl SkipXdr for #{name typedef} {
          #[cfg(feature = "std")]
          fn skip_xdr<R: Read>(r: &mut Limited<R>) -> Result<(), Error> {
              r.with_limited_depth(|r| #{reference_to_call(typedef, typedef.type)}::skip_xdr(r))
          }
      }

      impl SeekableSkipXdr for #{name typedef} {
          #[cfg(feature = "std")]
          fn seekable_skip_xdr<R: Read + Seek>(r: &mut Limited<R>) -> Result<(), Error> {
              r.with_limited_depth(|r| #{reference_to_call(typedef, typedef.type)}::seekable_skip_xdr(r))
          }
      }
      EOS
      if is_fixed_array_type(typedef.type)
        out.break
        out.puts <<-EOS.strip_heredoc
        impl #{name typedef} {
            #[must_use]
            pub fn as_slice(&self) -> &[#{element_type_for_vec(typedef.type)}] {
                &self.0
            }
        }

        #[cfg(feature = "alloc")]
        impl TryFrom<Vec<#{element_type_for_vec(typedef.type)}>> for #{name typedef} {
            type Error = Error;
            fn try_from(x: Vec<#{element_type_for_vec(typedef.type)}>) -> Result<Self, Error> {
                x.as_slice().try_into()
            }
        }

        #[cfg(feature = "alloc")]
        impl TryFrom<&Vec<#{element_type_for_vec(typedef.type)}>> for #{name typedef} {
            type Error = Error;
            fn try_from(x: &Vec<#{element_type_for_vec(typedef.type)}>) -> Result<Self, Error> {
                x.as_slice().try_into()
            }
        }

        impl TryFrom<&[#{element_type_for_vec(typedef.type)}]> for #{name typedef} {
            type Error = Error;
            fn try_from(x: &[#{element_type_for_vec(typedef.type)}]) -> Result<Self, Error> {
                Ok(#{name typedef}(x.try_into()?))
            }
        }

        impl AsRef<[#{element_type_for_vec(typedef.type)}]> for #{name typedef} {
            #[must_use]
            fn as_ref(&self) -> &[#{element_type_for_vec(typedef.type)}] {
                &self.0
            }
        }
        EOS
      end
      if is_var_array_type(typedef.type)
        out.break
        out.puts <<-EOS.strip_heredoc
        impl Deref for #{name typedef} {
          type Target = #{reference(typedef, typedef.type)};
          fn deref(&self) -> &Self::Target {
              &self.0
          }
        }

        impl From<#{name typedef}> for Vec<#{element_type_for_vec(typedef.type)}> {
            #[must_use]
            fn from(x: #{name typedef}) -> Self {
                x.0.0
            }
        }

        impl TryFrom<Vec<#{element_type_for_vec(typedef.type)}>> for #{name typedef} {
            type Error = Error;
            fn try_from(x: Vec<#{element_type_for_vec(typedef.type)}>) -> Result<Self, Error> {
                Ok(#{name typedef}(x.try_into()?))
            }
        }

        #[cfg(feature = "alloc")]
        impl TryFrom<&Vec<#{element_type_for_vec(typedef.type)}>> for #{name typedef} {
            type Error = Error;
            fn try_from(x: &Vec<#{element_type_for_vec(typedef.type)}>) -> Result<Self, Error> {
                Ok(#{name typedef}(x.try_into()?))
            }
        }

        impl AsRef<Vec<#{element_type_for_vec(typedef.type)}>> for #{name typedef} {
            #[must_use]
            fn as_ref(&self) -> &Vec<#{element_type_for_vec(typedef.type)}> {
                &self.0.0
            }
        }

        impl AsRef<[#{element_type_for_vec(typedef.type)}]> for #{name typedef} {
            #[cfg(feature = "alloc")]
            #[must_use]
            fn as_ref(&self) -> &[#{element_type_for_vec(typedef.type)}] {
                &self.0.0
            }
            #[cfg(not(feature = "alloc"))]
            #[must_use]
            fn as_ref(&self) -> &[#{element_type_for_vec(typedef.type)}] {
                self.0.0
            }
        }
        EOS
      end
    end
    out.break
  end

  def render_const(out, const)
    out.puts "pub const #{name(const).underscore.upcase}: u64 = #{const.value};"
    out.break
  end

  def is_builtin_type(type)
    [
      AST::Typespecs::Bool,
      AST::Typespecs::Double, AST::Typespecs::Float,
      AST::Typespecs::UnsignedHyper, AST::Typespecs::UnsignedInt,
      AST::Typespecs::Hyper, AST::Typespecs::Int,
    ].any? { |t| t === type }
  end

  def is_fixed_array_opaque(type)
    (AST::Typespecs::Opaque === type && type.fixed?)
  end

  def is_fixed_array_type(type)
    (AST::Typespecs::Opaque === type && type.fixed?) ||
    (type.sub_type == :array)
  end

  def is_var_array_type(type)
    (AST::Typespecs::Opaque === type && !type.fixed?) ||
    (AST::Typespecs::String === type) ||
    (type.sub_type == :var_array)
  end

  def base_reference(type)
    case type
    when AST::Typespecs::Bool
      'bool'
    when AST::Typespecs::Double
      $stderr.puts "warn: rust generator has not implemented f64 support"
      'f64'
    when AST::Typespecs::Float
      $stderr.puts "warn: rust generator has not implemented f64 support"
      'f32'
    when AST::Typespecs::UnsignedHyper
      'u64'
    when AST::Typespecs::UnsignedInt
      'u32'
    when AST::Typespecs::Hyper
      'i64'
    when AST::Typespecs::Int
      'i32'
    when AST::Typespecs::Quadruple
      raise 'no quadruple support for rust'
    when AST::Typespecs::String
      if !type.decl.resolved_size.nil?
        "StringM::<#{type.decl.resolved_size}>"
      else
        "StringM"
      end
    when AST::Typespecs::Opaque
      if type.fixed?
        "[u8; #{type.size}]"
      elsif !type.decl.resolved_size.nil?
        "BytesM::<#{type.decl.resolved_size}>"
      else
        "BytesM"
      end
    when AST::Typespecs::Simple, AST::Definitions::Base, AST::Concerns::NestedDefinition
      if type.respond_to?(:resolved_type) && AST::Definitions::Typedef === type.resolved_type && is_builtin_type(type.resolved_type.type)
        base_reference(type.resolved_type.type)
      else
        name type
      end
    else
      raise "Unknown reference type: #{type.class.name}, #{type.class.ancestors}"
    end
  end

  def array_size(type)
    _, size = type.array_size
    size = name @top.find_definition(size) if is_named
    size
  end

  def field_attrs(parent, type)
    base_ref = base_reference(type)
    if ['i64','u64'].include?(base_ref)
      ref = reference(parent, type, 'NumberOrString')
      "#[cfg_attr(all(feature = \"serde\", feature = \"alloc\"), serde_as(as = \"#{ref}\"))]"
    else
      nil
    end
  end

  def reference(parent, type, base_ref = nil)
    base_ref = base_reference(type) if base_ref.nil?

    parent_name = name(parent) if parent
    cyclic = is_type_in_type_field_types(base_ref, parent_name)

    case type.sub_type
    when :simple
      if cyclic
        "Box<#{base_ref}>"
      else
        base_ref
      end
    when :optional
      if cyclic
        "Option<Box<#{base_ref}>>"
      else
        "Option<#{base_ref}>"
      end
    when :array
      is_named, size = type.array_size
      size = name @top.find_definition(size) if is_named
      "[#{base_ref}; #{size}]"
    when :var_array
      if !type.decl.resolved_size.nil?
        "VecM<#{base_ref}, #{type.decl.resolved_size}>"
      else
        "VecM<#{base_ref}>"
      end
    else
      raise "Unknown sub_type: #{type.sub_type}"
    end
  end

  def element_type_for_vec(type)
    case type
    when AST::Typespecs::String
      "u8"
    when AST::Typespecs::Opaque
      "u8"
    when AST::Typespecs::Simple, AST::Definitions::Base, AST::Concerns::NestedDefinition
      if type.respond_to?(:resolved_type) && AST::Definitions::Typedef === type.resolved_type && is_builtin_type(type.resolved_type.type)
        base_reference(type.resolved_type.type)
      else
        name type
      end
    else
      raise "Unknown element type for vec: #{type.class.name}, #{type.class.ancestors}"
    end
  end

  def base_reference_to_call(type)
    case type
    when AST::Typespecs::String
      if !type.decl.resolved_size.nil?
        "StringM::<#{type.decl.resolved_size}>"
      else
        "StringM::<{ u32::MAX }>"
      end
    when AST::Typespecs::Opaque
      if type.fixed?
        "[u8; #{type.size}]"
      elsif !type.decl.resolved_size.nil?
        "BytesM::<#{type.decl.resolved_size}>"
      else
        "BytesM::<{ u32::MAX }>"
      end
    when AST::Typespecs::Simple, AST::Definitions::Base, AST::Concerns::NestedDefinition
      if type.respond_to?(:resolved_type) && AST::Definitions::Typedef === type.resolved_type && is_builtin_type(type.resolved_type.type)
        base_reference_to_call(type.resolved_type.type)
      else
        base_reference(type)
      end
    else
      base_reference(type)
    end
  end

  def reference_to_call(parent, type)
    base_ref = base_reference_to_call(type)

    parent_name = name(parent) if parent
    cyclic = is_type_in_type_field_types(base_ref, parent_name)

    ref = case type.sub_type
    when :simple
      if cyclic
        "Box<#{base_ref}>"
      else
        base_ref
      end
    when :optional
      if cyclic
        "Option::<Box<#{base_ref}>>"
      else
        "Option::<#{base_ref}>"
      end
    when :array
      is_named, size = type.array_size
      size = name @top.find_definition(size) if is_named
      "[#{base_ref}; #{size}]"
    when :var_array
      if !type.decl.resolved_size.nil?
        "VecM::<#{base_ref}, #{type.decl.resolved_size}>"
      else
        "VecM::<#{base_ref}>"
      end
    else
      raise "Unknown sub_type: #{type.sub_type}"
    end

    if ref.starts_with?("[") && ref.ends_with?("]")
      "<#{ref}>"
    elsif ref.starts_with?("Box<") && ref.ends_with?(">")
      "Box::#{ref.delete_prefix("Box")}"
    else
      ref
    end
  end

  def name(named)
    parent = name named.parent_defn if named.is_a?(AST::Concerns::NestedDefinition)

    base = if named.respond_to?(:name_short)
      named.name_short
    elsif named.respond_to?(:name)
      named.name
    else
      named.text_value
    end
    base = escape_name(base)
    "#{parent}#{base.underscore.camelize}"
  end

  def field_name(named)
    escape_name named.name.underscore
  end

  def escape_name(name)
    case name
    when 'type' then 'type_'
    when 'Error' then 'SError'
    else name
    end
  end

  # ============================================================================
  # Sparse Type Generation
  # ============================================================================
  #
  # Sparse types are decode-only types that extract only specific nested fields
  # from an XDR type, skipping over unneeded data during decode.

  def render_sparse_types(out)
    return unless @options[:sparse_types]

    @options[:sparse_types].each do |config|
      @rendered_sparse_types = Set.new  # Track rendered sparse types per base type
      render_sparse_type(out, config)
    end
  end

  def render_sparse_type(out, config)
    base_type_name = config[:base_type]
    sparse_name = config[:name]
    paths = config[:paths]

    base_defn = @type_definitions[base_type_name]
    unless base_defn
      $stderr.puts "warn: sparse type base '#{base_type_name}' not found, skipping"
      return
    end

    # Build a tree structure representing which paths to extract
    # The tree maps: variant/field name -> subtree or :leaf (for target fields)
    path_tree = build_path_tree(paths)

    out.break
    out.puts "// ============================================================================"
    out.puts "// Sparse type: #{sparse_name}"
    out.puts "// Base type: #{base_type_name}"
    out.puts "// Extracts: #{paths.join(', ')}"
    out.puts "// ============================================================================"
    out.break

    # Generate the sparse type and all nested sparse types
    render_sparse_type_recursive(out, base_defn, sparse_name, path_tree, sparse_name)
  end

  # Build a tree from paths like ["Tx.tx.operations", "TxV0.tx.operations"]
  # Result: { "Tx" => { "tx" => { "operations" => :leaf } }, "TxV0" => { "tx" => { "operations" => :leaf } } }
  def build_path_tree(paths)
    tree = {}
    paths.each do |path|
      parts = path.split('.')
      current = tree
      parts.each_with_index do |part, i|
        # Check for array traversal marker
        is_array_traversal = part.end_with?('[]')
        part = part.chomp('[]') if is_array_traversal

        if i == parts.length - 1
          # Leaf node - this is the target field to extract
          current[part] = { _leaf: true, _array: is_array_traversal }
        else
          current[part] ||= {}
          current[part][:_array] = true if is_array_traversal
          current = current[part]
        end
      end
    end
    tree
  end

  def render_sparse_type_recursive(out, defn, sparse_type_name, path_tree, prefix)
    # Skip if we've already rendered this sparse type
    if @rendered_sparse_types.include?(sparse_type_name)
      return
    end
    @rendered_sparse_types << sparse_type_name
    @sparse_types << sparse_type_name  # Add to global sparse types set

    case defn
    when AST::Definitions::Union
      render_sparse_union(out, defn, sparse_type_name, path_tree, prefix)
    when AST::Definitions::Struct
      render_sparse_struct(out, defn, sparse_type_name, path_tree, prefix)
    when AST::Definitions::Typedef
      # For typedef, unwrap and process the underlying type
      render_sparse_typedef(out, defn, sparse_type_name, path_tree, prefix)
    when AST::Definitions::Enum
      # Enums don't have nested fields, just reference the original
      $stderr.puts "warn: sparse type path leads to enum '#{name(defn)}', using original type"
    end
  end

  def render_sparse_union(out, union, sparse_type_name, path_tree, prefix)
    discriminant_type = reference(nil, union.discriminant.type)

    # Collect all cases and determine which have paths
    cases_info = []
    union_cases(union) do |case_name, arm, value|
      subtree = path_tree[case_name]
      cases_info << {
        case_name: case_name,
        arm: arm,
        value: value,
        subtree: subtree,
        has_path: !subtree.nil?
      }
    end

    out.puts "/// Sparse type for #{name(union)} extracting only specified paths"
    out.puts "#[allow(non_camel_case_types)]"
    out.puts "#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]"
    out.puts %{#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]}
    out.puts %{#[cfg_attr(all(feature = "serde", feature = "alloc"), derive(serde::Serialize, serde::Deserialize), serde(rename_all = "snake_case"))]}
    out.puts %{#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]}
    out.puts "pub enum #{sparse_type_name} {"
    out.indent do
      cases_info.each do |info|
        if info[:has_path] && !info[:arm].void?
          # This variant has a path - generate a nested sparse type
          nested_type_name = "#{prefix}_#{info[:case_name]}"
          out.puts "#{info[:case_name]}(#{nested_type_name}),"
        else
          # No path through this variant - make it a unit variant
          out.puts "#{info[:case_name]},"
        end
      end
    end
    out.puts "}"
    out.puts ""

    # Generate ReadXdr for the sparse union
    out.puts <<-EOS.strip_heredoc
    impl ReadXdr for #{sparse_type_name} {
        #[cfg(feature = "std")]
        fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
            r.with_limited_depth(|r| {
                let dv: #{discriminant_type} = <#{discriminant_type} as ReadXdr>::read_xdr(r)?;
                #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
                let v = match dv {
    EOS

    out.indent(5) do
      cases_info.each do |info|
        discriminant_match = info[:value].nil? ? "#{discriminant_type}::#{info[:case_name]}" : "#{info[:value]}"

        if info[:has_path] && !info[:arm].void?
          nested_type_name = "#{prefix}_#{info[:case_name]}"
          out.puts "#{discriminant_match} => Self::#{info[:case_name]}(#{nested_type_name}::read_xdr(r)?),"
        else
          # Skip the arm's data if it's not void
          unless info[:arm].void?
            out.puts "#{discriminant_match} => { #{reference_to_call(union, info[:arm].type)}::skip_xdr(r)?; Self::#{info[:case_name]} },"
          else
            out.puts "#{discriminant_match} => Self::#{info[:case_name]},"
          end
        end
      end
      out.puts "#[allow(unreachable_patterns)]"
      out.puts "_ => return Err(Error::Invalid),"
    end

    out.puts <<-EOS.strip_heredoc
                };
                Ok(v)
            })
        }
    }
    EOS
    out.break

    # Generate SeekableReadXdr for the sparse union
    out.puts <<-EOS.strip_heredoc
    impl SeekableReadXdr for #{sparse_type_name} {
        #[cfg(feature = "std")]
        fn seekable_read_xdr<R: Read + Seek>(r: &mut Limited<R>) -> Result<Self, Error> {
            r.with_limited_depth(|r| {
                let dv: #{discriminant_type} = <#{discriminant_type} as ReadXdr>::read_xdr(r)?;
                #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
                let v = match dv {
    EOS

    out.indent(5) do
      cases_info.each do |info|
        discriminant_match = info[:value].nil? ? "#{discriminant_type}::#{info[:case_name]}" : "#{info[:value]}"

        if info[:has_path] && !info[:arm].void?
          nested_type_name = "#{prefix}_#{info[:case_name]}"
          out.puts "#{discriminant_match} => Self::#{info[:case_name]}(#{nested_type_name}::seekable_read_xdr(r)?),"
        else
          # Skip the arm's data if it's not void
          unless info[:arm].void?
            out.puts "#{discriminant_match} => { #{reference_to_call(union, info[:arm].type)}::seekable_skip_xdr(r)?; Self::#{info[:case_name]} },"
          else
            out.puts "#{discriminant_match} => Self::#{info[:case_name]},"
          end
        end
      end
      out.puts "#[allow(unreachable_patterns)]"
      out.puts "_ => return Err(Error::Invalid),"
    end

    out.puts <<-EOS.strip_heredoc
                };
                Ok(v)
            })
        }
    }
    EOS
    out.break

    # Now generate nested sparse types for variants with paths
    cases_info.each do |info|
      next unless info[:has_path] && !info[:arm].void?

      nested_type_name = "#{prefix}_#{info[:case_name]}"
      # Get the actual type definition for this arm
      arm_type_ref = base_reference(info[:arm].type)
      arm_defn = @type_definitions[arm_type_ref]

      if arm_defn
        # Remove the _leaf and _array markers when passing subtree down
        clean_subtree = info[:subtree].reject { |k, _| k.is_a?(Symbol) }
        render_sparse_type_recursive(out, arm_defn, nested_type_name, clean_subtree, prefix)
      else
        $stderr.puts "warn: could not find definition for arm type '#{arm_type_ref}'"
      end
    end
  end

  def render_sparse_struct(out, struct, sparse_type_name, path_tree, prefix)
    # Determine which fields to include and which to skip
    fields_info = struct.members.map do |m|
      field_n = field_name(m)
      subtree = path_tree[field_n]
      {
        member: m,
        field_name: field_n,
        subtree: subtree,
        is_leaf: subtree && subtree[:_leaf],
        is_array_traversal: subtree && subtree[:_array],
        has_path: !subtree.nil?
      }
    end

    # Fields to include in the sparse struct
    included_fields = fields_info.select { |f| f[:has_path] }

    out.puts "/// Sparse type for #{name(struct)} extracting only specified paths"
    out.puts "#[allow(non_camel_case_types)]"
    out.puts "#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]"
    out.puts %{#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]}
    out.puts %{#[cfg_attr(all(feature = "serde", feature = "alloc"), derive(serde::Serialize, serde::Deserialize), serde(rename_all = "snake_case"))]}
    out.puts %{#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]}
    out.puts "pub struct #{sparse_type_name} {"
    out.indent do
      included_fields.each do |info|
        m = info[:member]
        if info[:is_leaf]
          # This is a target field - use the original type
          if info[:is_array_traversal]
            # For array traversal, we keep the Vec but with sparse inner type
            # For now, just keep the full type at the leaf
            out.puts "pub #{info[:field_name]}: #{reference(struct, m.declaration.type)},"
          else
            out.puts "pub #{info[:field_name]}: #{reference(struct, m.declaration.type)},"
          end
        else
          # This is an intermediate field - use a nested sparse type
          nested_type_name = "#{prefix}_#{name(struct).gsub(prefix + '_', '')}_#{info[:field_name].camelize}"
          if info[:is_array_traversal]
            # This is an array traversal - the sparse type wraps each element
            inner_type = element_type_for_sparse_array(m.declaration.type)
            vec_max = get_vec_max(m.declaration.type)
            if vec_max
              out.puts "pub #{info[:field_name]}: VecM<#{nested_type_name}, #{vec_max}>,"
            else
              out.puts "pub #{info[:field_name]}: VecM<#{nested_type_name}>,"
            end
          else
            out.puts "pub #{info[:field_name]}: #{nested_type_name},"
          end
        end
      end
    end
    out.puts "}"
    out.puts ""

    # Generate ReadXdr for the sparse struct
    out.puts "impl ReadXdr for #{sparse_type_name} {"
    out.puts "    #[cfg(feature = \"std\")]"
    out.puts "    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {"
    out.puts "        r.with_limited_depth(|r| {"

    # Process each field in order
    fields_info.each do |info|
      m = info[:member]
      if info[:has_path]
        if info[:is_leaf]
          # Read the target field
          out.puts "            let #{info[:field_name]} = #{reference_to_call(struct, m.declaration.type)}::read_xdr(r)?;"
        else
          # Read through a nested sparse type
          nested_type_name = "#{prefix}_#{name(struct).gsub(prefix + '_', '')}_#{info[:field_name].camelize}"
          if info[:is_array_traversal]
            # Read array length, then read each element with sparse type
            vec_max = get_vec_max(m.declaration.type) || "{ u32::MAX }"
            out.puts "            let #{info[:field_name]} = {"
            out.puts "                let len = u32::read_xdr(r)?;"
            out.puts "                if len > #{vec_max} { return Err(Error::LengthExceedsMax); }"
            out.puts "                let mut vec = Vec::new();"
            out.puts "                for _ in 0..len {"
            out.puts "                    vec.push(#{nested_type_name}::read_xdr(r)?);"
            out.puts "                }"
            out.puts "                VecM::try_from(vec).map_err(|_| Error::LengthExceedsMax)?"
            out.puts "            };"
          else
            out.puts "            let #{info[:field_name]} = #{nested_type_name}::read_xdr(r)?;"
          end
        end
      else
        # Skip this field
        out.puts "            #{reference_to_call(struct, m.declaration.type)}::skip_xdr(r)?;"
      end
    end

    out.puts "            Ok(Self {"
    included_fields.each do |info|
      out.puts "                #{info[:field_name]},"
    end
    out.puts "            })"
    out.puts "        })"
    out.puts "    }"
    out.puts "}"
    out.break

    # Generate SeekableReadXdr for the sparse struct
    out.puts "impl SeekableReadXdr for #{sparse_type_name} {"
    out.puts "    #[cfg(feature = \"std\")]"
    out.puts "    fn seekable_read_xdr<R: Read + Seek>(r: &mut Limited<R>) -> Result<Self, Error> {"
    out.puts "        r.with_limited_depth(|r| {"

    # Process each field in order
    fields_info.each do |info|
      m = info[:member]
      if info[:has_path]
        if info[:is_leaf]
          # Read the target field
          out.puts "            let #{info[:field_name]} = #{reference_to_call(struct, m.declaration.type)}::read_xdr(r)?;"
        else
          # Read through a nested sparse type
          nested_type_name = "#{prefix}_#{name(struct).gsub(prefix + '_', '')}_#{info[:field_name].camelize}"
          if info[:is_array_traversal]
            # Read array length, then read each element with sparse type
            vec_max = get_vec_max(m.declaration.type) || "{ u32::MAX }"
            out.puts "            let #{info[:field_name]} = {"
            out.puts "                let len = u32::read_xdr(r)?;"
            out.puts "                if len > #{vec_max} { return Err(Error::LengthExceedsMax); }"
            out.puts "                let mut vec = Vec::new();"
            out.puts "                for _ in 0..len {"
            out.puts "                    vec.push(#{nested_type_name}::seekable_read_xdr(r)?);"
            out.puts "                }"
            out.puts "                VecM::try_from(vec).map_err(|_| Error::LengthExceedsMax)?"
            out.puts "            };"
          else
            out.puts "            let #{info[:field_name]} = #{nested_type_name}::seekable_read_xdr(r)?;"
          end
        end
      else
        # Skip this field using seek
        out.puts "            #{reference_to_call(struct, m.declaration.type)}::seekable_skip_xdr(r)?;"
      end
    end

    out.puts "            Ok(Self {"
    included_fields.each do |info|
      out.puts "                #{info[:field_name]},"
    end
    out.puts "            })"
    out.puts "        })"
    out.puts "    }"
    out.puts "}"
    out.break

    # Generate nested sparse types for intermediate fields
    included_fields.each do |info|
      next if info[:is_leaf]

      m = info[:member]
      nested_type_name = "#{prefix}_#{name(struct).gsub(prefix + '_', '')}_#{info[:field_name].camelize}"

      # Get the type definition for this field
      if info[:is_array_traversal]
        # Get the element type of the array
        field_type_ref = element_type_for_sparse_array(m.declaration.type)
      else
        field_type_ref = base_reference(m.declaration.type)
      end

      field_defn = @type_definitions[field_type_ref]

      if field_defn
        # Remove the _leaf and _array markers when passing subtree down
        clean_subtree = info[:subtree].reject { |k, _| k.is_a?(Symbol) }
        render_sparse_type_recursive(out, field_defn, nested_type_name, clean_subtree, prefix)
      else
        $stderr.puts "warn: could not find definition for field type '#{field_type_ref}'"
      end
    end
  end

  def render_sparse_typedef(out, typedef, sparse_type_name, path_tree, prefix)
    # For typedef, we need to look at the underlying type
    underlying_type_ref = base_reference(typedef.type)
    underlying_defn = @type_definitions[underlying_type_ref]

    if underlying_defn
      render_sparse_type_recursive(out, underlying_defn, sparse_type_name, path_tree, prefix)
    else
      # The underlying type might be a primitive - just generate a wrapper
      out.puts "/// Sparse type for #{name(typedef)}"
      out.puts "pub type #{sparse_type_name} = #{underlying_type_ref};"
      out.break
    end
  end

  # Get the element type for a VecM or array type
  def element_type_for_sparse_array(type)
    case type.sub_type
    when :var_array
      base_reference(type)
    when :array
      base_reference(type)
    else
      base_reference(type)
    end
  end

  # Get the max size for a VecM type
  def get_vec_max(type)
    if type.sub_type == :var_array && !type.decl.resolved_size.nil?
      type.decl.resolved_size.to_s
    else
      nil
    end
  end

end
