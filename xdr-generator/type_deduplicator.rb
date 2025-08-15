class TypeDeduplicator
  def initialize(curr_file:, next_file:)
    @curr_file = curr_file
    @next_file = next_file
    @curr_content = File.read(curr_file)
    @next_content = File.read(next_file)
    @curr_types = extract_types(@curr_content)
    @next_types = extract_types(@next_content)
    @type_dependencies = extract_type_dependencies(@next_content)
  end

  def generate_deduplicated_next
    different_types = find_different_types
    types_to_include = compute_types_to_include(different_types)
    
    # Generate header content from next
    header = extract_header(@next_content)
    
    # Generate re-exports for identical types
    reexports = generate_reexports(types_to_include)
    
    # Generate definitions for different types only
    definitions = generate_definitions_for_types(types_to_include)
    
    # Generate the Type enum with all types (both re-exported and defined)
    type_enum = generate_type_enum
    
    [header, reexports, definitions, type_enum].join("\n")
  end

  private

  def extract_header(content)
    # Extract everything up to the first type definition
    lines = content.lines
    header_lines = []
    
    lines.each do |line|
      # Stop when we hit the first actual type definition (not Type enum)
      if line.match?(/^#\[derive.*\]$/) && 
         lines[lines.index(line) + 1]&.match?(/^pub (struct|enum|union|type) (?!Type\b)/)
        break
      end
      header_lines << line
    end
    
    header_lines.join.strip
  end

  def extract_types(content)
    types = {}
    current_type = nil
    current_content = []
    in_type = false
    brace_count = 0
    indent_level = 0
    
    content.lines.each_with_index do |line, line_num|
      stripped = line.strip
      
      # Detect type definition start
      if line.match?(/^(#\[.*?\]\s*)*(pub\s+)?(struct|enum|union|type)\s+(\w+)/)
        # Save previous type if we were processing one
        if current_type && in_type && !current_content.empty?
          types[current_type] = current_content.join.strip
        end
        
        # Extract type name
        type_match = line.match(/^(#\[.*?\]\s*)*(pub\s+)?(struct|enum|union|type)\s+(\w+)/)
        type_name = type_match[4] if type_match
        
        # Skip the main Type enum as it's handled separately
        if type_name == "Type"
          current_type = nil
          in_type = false
          current_content = []
          next
        end
        
        # Start new type
        current_type = type_name
        current_content = [line]
        in_type = true
        brace_count = line.count('{') - line.count('}')
        indent_level = line.index(/\S/) || 0
      elsif in_type && current_type
        current_content << line
        brace_count += line.count('{') - line.count('}')
        
        # Type definition ends when:
        # 1. Braces are balanced AND
        # 2. We hit an empty line OR next non-empty line starts a new definition
        if brace_count <= 0
          # Look ahead to see if next non-empty line starts a new type
          remaining_lines = content.lines[(line_num + 1)..-1] || []
          next_meaningful_line = remaining_lines.find { |l| !l.strip.empty? }
          
          if next_meaningful_line.nil? || 
             next_meaningful_line.match?(/^(#\[.*?\]\s*)*(pub\s+)?(struct|enum|union|type)\s+\w+/) ||
             stripped.empty?
            types[current_type] = current_content.join.strip
            current_type = nil
            in_type = false
            current_content = []
          end
        end
      end
    end
    
    # Handle last type if file doesn't end properly
    if current_type && in_type && !current_content.empty?
      types[current_type] = current_content.join.strip
    end
    
    types
  end

  def extract_type_dependencies(content)
    dependencies = Hash.new { |h, k| h[k] = Set.new }
    
    @next_types.each do |type_name, type_content|
      # Find all type references in this type's definition
      type_content.scan(/\b([A-Z][a-zA-Z0-9_]*)\b/).flatten.each do |referenced_type|
        # Only track dependencies on types we know about
        if @next_types.key?(referenced_type) && referenced_type != type_name
          dependencies[type_name].add(referenced_type)
        end
      end
    end
    
    dependencies
  end

  def find_different_types
    different = Set.new
    
    @next_types.each do |type_name, next_definition|
      curr_definition = @curr_types[type_name]
      
      if curr_definition.nil?
        # Type doesn't exist in curr, it's new
        different.add(type_name)
      else
        # Normalize definitions for comparison (remove comments, whitespace differences)
        normalized_curr = normalize_type_definition(curr_definition)
        normalized_next = normalize_type_definition(next_definition)
        
        if normalized_curr != normalized_next
          different.add(type_name)
        end
      end
    end
    
    different
  end

  def normalize_type_definition(definition)
    # Remove comments and normalize whitespace for comparison
    normalized = definition.gsub(/\/\/.*$/, '')  # Remove line comments
    normalized = normalized.gsub(/\/\*.*?\*\//m, '')  # Remove block comments
    
    # Remove attributes (like #[derive(...)])
    normalized = normalized.gsub(/^\s*#\[.*?\]\s*$/m, '')
    
    # Normalize whitespace
    normalized = normalized.gsub(/\s+/, ' ')
    normalized = normalized.strip
    normalized
  end

  def compute_types_to_include(different_types)
    # Start with directly different types
    to_include = different_types.dup
    
    # Add types that depend on different types (transitively)
    changed = true
    while changed
      changed = false
      @type_dependencies.each do |type_name, deps|
        next if to_include.include?(type_name)
        
        # If this type depends on any type that we're including, include it too
        if deps.any? { |dep| to_include.include?(dep) }
          to_include.add(type_name)
          changed = true
        end
      end
    end
    
    to_include
  end

  def generate_reexports(types_to_include)
    reexport_types = @next_types.keys - types_to_include.to_a
    
    return "" if reexport_types.empty?
    
    reexports = ["// Re-exported types from curr that are identical"]
    reexport_types.sort.each do |type_name|
      reexports << "pub use crate::curr::generated::#{type_name};"
    end
    reexports << ""
    
    reexports.join("\n")
  end

  def generate_definitions_for_types(types_to_include)
    return "" if types_to_include.empty?
    
    definitions = ["// Types that are different from curr"]
    types_to_include.sort.each do |type_name|
      if @next_types[type_name]
        definitions << @next_types[type_name].strip
        definitions << ""
      end
    end
    
    definitions.join("\n")
  end

  def generate_type_enum
    # Extract the Type enum from the next content
    lines = @next_content.lines
    enum_lines = []
    in_enum = false
    brace_count = 0
    
    lines.each do |line|
      if line.match?(/^pub enum Type\b/)
        in_enum = true
        enum_lines << line
        brace_count = line.count('{') - line.count('}')
      elsif in_enum
        enum_lines << line
        brace_count += line.count('{') - line.count('}')
        
        if brace_count == 0 && line.strip.empty?
          break
        end
      end
    end
    
    enum_lines.join
  end
end