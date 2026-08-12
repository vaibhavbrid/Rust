#[derive(Debug)]
enum NetworkError {
    Disconnected,
    Timeout,
}

#[derive(Debug)]
enum CustomError {
    NotFound,
    Network(NetworkError),
}

impl From<NetworkError> for CustomError {
    fn from(err: NetworkError) -> Self {
        CustomError::Network(err)
    }
}

fn simulate_network() -> Result<(), NetworkError> {
    Err(NetworkError::Disconnected)
}

fn main() -> Result<(), CustomError> {
    simulate_network()?;
    Ok(())
}
