use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ErrCodes {
    // System Service
    SystemGetRamInfoParseError,
    SystemGetRamInfoInvalidResponse,
    SystemGetRamInfoInvokeError,
    // Config Service
    ConfigGetConfigMissingParamKey,
    ConfigGetConfigInvalidResponse,
    ConfigGetConfigInvokeError,
    ConfigSetConfigMssingParamKey,
    ConfigSetConfigMssingParamValue,
    ConfigSetConfigMssingParamKeyAndValue,
    ConfigSetConfigInvalidPayloadFormat,
    ConfigSetConfigInvalidResponse,
    ConfigSetConfigInvokeError,
    // HttpClient Service
    HttpClientRequestMissingParamReq,
    HttpClientRequestInvalidResponse,
    HttpClientRequestInvokeError,
    HttpClientGetMissingParamUrl,
    HttpClientGetInvalidPayloadFormat,
    HttpClientPostMissingParamUrl,
    HttpClientPostInvalidPayloadFormat,
    HttpClientPutMissingParamUrl,
    HttpClientPutInvalidPayloadFormat,
    HttpClientDeleteMissingParamUrl,
    HttpClientDeleteInvalidPayloadFormat,
    HttpClientPatchMissingParamUrl,
    HttpClientPatchInvalidPayloadFormat,
    // HttpServer Service
    HttpServerStartInvalidPort,
    HttpServerStartMissingParamPort,
    HttpServerStartMissingParamLang,
    HttpServerStartMissingParamPortAndLang,
    HttpServerStartInvalidPayloadFormat,
    HttpServerStartParseResponseError,
    HttpServerStartInvalidResponse,
    HttpServerStartInvokeError,
    HttpServerStopParseResponseError,
    HttpServerStopInvalidResponse,
    HttpServerStopInvokeError,
    HttpServerStatusParseResponseError,
    HttpServerStatusInvalidResponse,
    HttpServerStatusInvokeError,
}
