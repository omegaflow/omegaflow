pub enum Magic {
    Pdf,
    Zip,
    Fits,
    Png,
    Gzip,
    Hdf5,
    NetCdf,
    Tiff,
    Unrecognized,
}

pub fn magic_identity(bytes: &[u8]) -> Magic {
    if bytes.starts_with(b"%PDF") {
        Magic::Pdf
    } else if bytes.starts_with(b"PK\x03\x04") || bytes.starts_with(b"PK\x05\x06") {
        Magic::Zip
    } else if bytes.starts_with(b"SIMPLE") {
        Magic::Fits
    } else if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        Magic::Png
    } else if bytes.starts_with(b"\x1f\x8b") {
        Magic::Gzip
    } else if bytes.starts_with(b"\x89HDF\r\n\x1a\n") {
        Magic::Hdf5
    } else if bytes.starts_with(b"CDF\x01") || bytes.starts_with(b"CDF\x02") {
        Magic::NetCdf
    } else if bytes.starts_with(b"II*\x00") || bytes.starts_with(b"MM\x00*") {
        Magic::Tiff
    } else {
        Magic::Unrecognized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pdf() {
        assert!(matches!(magic_identity(b"%PDF-1.7\n"), Magic::Pdf));
    }

    #[test]
    fn zip() {
        assert!(matches!(magic_identity(b"PK\x03\x04rest"), Magic::Zip));
        assert!(matches!(magic_identity(b"PK\x05\x06"), Magic::Zip));
    }

    #[test]
    fn fits() {
        assert!(matches!(
            magic_identity(b"SIMPLE  =                    T"),
            Magic::Fits
        ));
    }

    #[test]
    fn png() {
        assert!(matches!(
            magic_identity(b"\x89PNG\r\n\x1a\nrest"),
            Magic::Png
        ));
    }

    #[test]
    fn gzip() {
        assert!(matches!(magic_identity(b"\x1f\x8b\x08"), Magic::Gzip));
    }

    #[test]
    fn hdf5() {
        assert!(matches!(
            magic_identity(b"\x89HDF\r\n\x1a\nrest"),
            Magic::Hdf5
        ));
    }

    #[test]
    fn netcdf() {
        assert!(matches!(magic_identity(b"CDF\x01rest"), Magic::NetCdf));
        assert!(matches!(magic_identity(b"CDF\x02rest"), Magic::NetCdf));
    }

    #[test]
    fn tiff() {
        assert!(matches!(magic_identity(b"II*\x00rest"), Magic::Tiff));
        assert!(matches!(magic_identity(b"MM\x00*rest"), Magic::Tiff));
    }

    #[test]
    fn unrecognized() {
        assert!(matches!(magic_identity(b"hello"), Magic::Unrecognized));
    }

    #[test]
    fn empty_and_short() {
        assert!(matches!(magic_identity(b""), Magic::Unrecognized));
        assert!(matches!(magic_identity(b"PK"), Magic::Unrecognized));
        assert!(matches!(magic_identity(b"\x1f"), Magic::Unrecognized));
    }
}
