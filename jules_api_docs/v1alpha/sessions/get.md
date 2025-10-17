# https://developers.google.com/jules/api/reference/rest/v1alpha/sessions/get

  * HTTP request
  * Path parameters
  * Request body
  * Response body



Gets a single session.

### HTTP request

`GET https://jules.googleapis.com/v1alpha/{name=sessions/*}`

The URL uses [gRPC Transcoding](https://google.aip.dev/127) syntax.

### Path parameters

Parameters  
---  
`name` |  `string` Required. The resource name of the session to retrieve. Format: sessions/{session} It takes the form `sessions/{session}`.  
  
### Request body

The request body must be empty.

### Response body

If successful, the response body contains an instance of `[Session](/jules/api/reference/rest/v1alpha/sessions#Session)`.
