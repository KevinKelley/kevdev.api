// use std::fmt;

// CustomError::new(500, format!("Failed getting db connection: {}", e))

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct CustomError {
    pub code: u32,
    pub message: String
}

// impl new() for CustomError {

// }

// impl fmt::Display for CustomError {
    
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "status code: {}: {}", self.code, self.message);
//     }
// }
