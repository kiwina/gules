# https://developers.google.com/jules/api/reference/rest/v1alpha/sources/get

  * HTTP request  
  * Path parameters
  * Request body
  * Response body



Gets a single source.

### HTTP request

`GET https://jules.googleapis.com/v1alpha/{name=sources/**}`

The URL uses [gRPC Transcoding](https://google.aip.dev/127) syntax.

### Path parameters

Parameters  
---  
`name` |  `string` Required. The resource name of the source to retrieve. Format: sources/{source} It takes the form `sources/{+source}`.  
  
### Request body

The request body must be empty.

### Response body

If successful, the response body contains an instance of `[Source](/jules/api/reference/rest/v1alpha/sources#Source)`.
