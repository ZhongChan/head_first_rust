use handle_errors::Error;
use std::collections::HashMap;

/// Pagination struct that is getting extracted
/// from query params
#[derive(Debug, Default)]
pub struct Pagination {
    /// The index to the first item that has to be retured
    pub limit: Option<u32>,

    /// The index to the last item that has to be retured
    pub offset: u32,
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
/// query.insert("limit".to_string(),"1".to_string());
/// query.insert("offset".to_string(),"3".to_string());
/// let p = types::pagination::extract_pagination(query).unwarp();
/// assert_eq!(p.limit,1);
/// assert_eq!(p.offset,3);
/// ```
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    // Could be improved in the future
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            // Takes the "limit" parameter in the query
            // and tries to convert it to a number
            limit: Some(
                params
                    .get("limit")
                    .unwrap()
                    .parse::<u32>()
                    .map_err(Error::ParseError)?,
            ),
            // Takes the "offset" parameter in the query
            // and tries to convert it to a number
            offset: params
                .get("offset")
                .unwrap()
                .parse::<u32>()
                .map_err(Error::ParseError)?,
        });
    }
    Err(Error::MissingParameters)
}
