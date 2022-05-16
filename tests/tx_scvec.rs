#[cfg(feature = "std")]
use stellar_xdr::*;

#[cfg(feature = "std")]
#[test]
fn test_scvec() -> Result<(), Error> {
    let v: ScVec = vec![ScVal::ScvI32(2)].try_into()?;
    let xdr = v.to_xdr_base64()?;
    println!("{}", xdr);
    Ok(())
}
