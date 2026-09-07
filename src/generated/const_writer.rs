#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes an optional `i64`, mirroring `<Option<i64> as WriteXdr>::write_xdr`.
    pub const fn write_option_i64(&mut self, v: &Option<i64>) {
        match v {
            Some(v) => {
                self.write_u32(1);
                self.write_i64(*v);
            }
            None => {
                self.write_u32(0);
            }
        }
    }

    /// Serializes an optional `u32`, mirroring `<Option<u32> as WriteXdr>::write_xdr`.
    pub const fn write_option_u32(&mut self, v: &Option<u32>) {
        match v {
            Some(v) => {
                self.write_u32(1);
                self.write_u32(*v);
            }
            None => {
                self.write_u32(0);
            }
        }
    }

    /// Serializes a variable-length array of `u32`, mirroring `<VecM<u32, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_vec_u32<const MAX: u32>(&mut self, v: &VecMRef<'_, u32, MAX>) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_u32(s[i]);
            i += 1;
        }
    }

    /// Serializes a variable-length array of `u64`, mirroring `<VecM<u64, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_vec_u64<const MAX: u32>(&mut self, v: &VecMRef<'_, u64, MAX>) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_u64(s[i]);
            i += 1;
        }
    }
}
