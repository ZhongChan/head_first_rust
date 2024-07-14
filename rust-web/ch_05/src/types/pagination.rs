use handle_errors::Error;
use std::collections::HashMap;

/// Pagination struct that is getting extracted
/// from query params
#[derive(Debug)]
pub struct Pagination {
    /// The index to the first item that has to be retured
    pub start: usize,

    /// The index to the last item that has to be retured
    pub end: usize,
}

/// Extract query params from the `/questions` route
/// # Example query
/// GET requests to this route can hanve a pagination attaced so we just
/// return the questions we need
/// `/questions?start=1&end=10`
///
/// # Example usage
/// ```rust
/// let mut query = HashMap::new();
/// query.insert("start".to_string(),"1".to_string());
/// query.insert("end".to_string(),"3".to_string());
/// let p = types::pagination::extract_pagination(query).unwarp();
/// assert_eq!(p.start,1);
/// assert_eq!(p.end,3);
/// ```
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    // Could be improved in the future
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            // Takes the "start" parameter in the query
            // and tries to convert it to a number
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
            // Takes the "end" parameter in the query
            // and tries to convert it to a number
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
        });
    }
    Err(Error::MissingParameters)
}
