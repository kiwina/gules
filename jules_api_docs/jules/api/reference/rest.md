# https://developers.google.com/jules/api/reference/rest

Programmatically create and access your asynchronous coding tasks.

  * REST Resource: v1alpha.sessions
  * REST Resource: v1alpha.sessions.activities
  * REST Resource: v1alpha.sources



## Service: jules.googleapis.com

To call this service, we recommend that you use the Google-provided [client libraries](https://cloud.google.com/apis/docs/client-libraries-explained). If your application needs to use your own libraries to call this service, use the following information when you make the API requests.

### Service endpoint

A [service endpoint](https://cloud.google.com/apis/design/glossary#api_service_endpoint) is a base URL that specifies the network address of an API service. One service might have multiple service endpoints. This service has the following service endpoint and all URIs below are relative to this service endpoint:

  * `https://jules.googleapis.com`



## REST Resource: [v1alpha.sessions](/jules/api/reference/rest/v1alpha/sessions)

Methods  
---  
`[approvePlan](/jules/api/reference/rest/v1alpha/sessions/approvePlan)` |  `POST /v1alpha/{session=sessions/*}:approvePlan`   
Approves a plan in a session.  
`[create](/jules/api/reference/rest/v1alpha/sessions/create)` |  `POST /v1alpha/sessions`   
Creates a new session.  
`[get](/jules/api/reference/rest/v1alpha/sessions/get)` |  `GET /v1alpha/{name=sessions/*}`   
Gets a single session.  
`[list](/jules/api/reference/rest/v1alpha/sessions/list)` |  `GET /v1alpha/sessions`   
Lists all sessions.  
`[sendMessage](/jules/api/reference/rest/v1alpha/sessions/sendMessage)` |  `POST /v1alpha/{session=sessions/*}:sendMessage`   
Sends a message from the user to a session.  
  
## REST Resource: [v1alpha.sessions.activities](/jules/api/reference/rest/v1alpha/sessions.activities)

Methods  
---  
`[get](/jules/api/reference/rest/v1alpha/sessions.activities/get)` |  `GET /v1alpha/{name=sessions/*/activities/*}`   
Gets a single activity.  
`[list](/jules/api/reference/rest/v1alpha/sessions.activities/list)` |  `GET /v1alpha/{parent=sessions/*}/activities`   
Lists activities for a session.  
  
## REST Resource: [v1alpha.sources](/jules/api/reference/rest/v1alpha/sources)

Methods  
---  
`[get](/jules/api/reference/rest/v1alpha/sources/get)` |  `GET /v1alpha/{name=sources/**}`   
Gets a single source.  
`[list](/jules/api/reference/rest/v1alpha/sources/list)` |  `GET /v1alpha/sources`   
Lists sources.
