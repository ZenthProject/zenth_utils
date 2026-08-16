
#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use zenth_utils::utils::sanitizer::parser::{
        FileType,
        FileParser
    };

    #[test]
    fn test_file_type_from_extension() {
        let path = PathBuf::from("test.mp3");
        assert_eq!(FileType::from_extension(&path).unwrap(), FileType::Mp3);

        let path = PathBuf::from("test.jpg");
        assert_eq!(FileType::from_extension(&path).unwrap(), FileType::Jpeg);

        let path = PathBuf::from("test.JPEG");
        assert_eq!(FileType::from_extension(&path).unwrap(), FileType::Jpeg);
    }

    #[test]
    fn test_file_type_from_signature_mp3() {
        let data_id3 = b"ID3\x04\x00\x00\x00\x00\x00\x00";
        assert_eq!(
            FileType::from_signature(data_id3).unwrap(),
            FileType::Mp3
        );

        let data_frame = [0xFF, 0xFB, 0x90, 0x00];
        assert_eq!(
            FileType::from_signature(&data_frame).unwrap(),
            FileType::Mp3
        );
    }

    #[test]
    fn test_file_type_from_signature_jpeg() {
        let data = [0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10];
        assert_eq!(FileType::from_signature(&data).unwrap(), FileType::Jpeg);
    }

    #[test]
    fn test_file_type_from_signature_png() {
        let data = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        assert_eq!(FileType::from_signature(&data).unwrap(), FileType::Png);
    }

    #[test]
    fn test_file_type_from_signature_pdf() {
        let data = b"%PDF-1.4\n";
        assert_eq!(FileType::from_signature(data).unwrap(), FileType::Pdf);
    }

    #[test]
    fn test_parse_validates_signature() {
        let path = PathBuf::from("test.mp3");
        let invalid_data = b"NOT AN MP3";
        assert!(FileParser::parse(&path, invalid_data).is_err());
    }

    #[test]
    fn test_analyze() {
        let path = PathBuf::from("test.mp3");
        let data = [0xFF, 0xFB, 0x90, 0x00];
        let info = FileParser::analyze(&path, &data).unwrap();
        assert_eq!(info.file_type, FileType::Mp3);
        assert_eq!(info.size, 4);
        assert!(info.signature_valid);
    }
}
