// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TryFrom<String> for Status {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.to_lowercase() == "todo" {
            Ok(Status::ToDo)
        } else if value.to_lowercase() == "inprogress" {
            Ok(Status::InProgress)
        } else if value.to_lowercase() == "done" {
            Ok(Status::Done)
        } else {
            Err("Invalid".to_string())
        }
    }
}
impl TryFrom<&str> for Status {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.to_lowercase() == "todo" {
            Ok(Status::ToDo)
        } else if value.to_lowercase() == "inprogress" {
            Ok(Status::InProgress)
        } else if value.to_lowercase() == "done" {
            Ok(Status::Done)
        } else {
            Err("Invalid".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}
