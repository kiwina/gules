# https://developers.google.com/jules/api/reference/rest/v1alpha/sessions/approvePlan

  * HTTP request  
  * Path parameters
  * Request body
  * Response body



Approves a plan in a session.

### HTTP request

`POST https://jules.googleapis.com/v1alpha/{session=sessions/*}:approvePlan`

The URL uses [gRPC Transcoding](https://google.aip.dev/127) syntax.

### Path parameters

Parameters  
---  
`session` |  `string` Required. The resource name of the session to approve the plan in. Format: sessions/{session} It takes the form `sessions/{session}`.  
  
### Request body

The request body must be empty.

### Response body

If successful, the response body is empty.
