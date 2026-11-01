export const getAppMeta=async()=>({version:'0.3.0',build_hash:'ui-fixture'});
export const getInternalApiStatus=async()=>({running:false,host:'127.0.0.1',port:61001});
export const invokeCommand=async()=>({running:false,host:'127.0.0.1',port:0});
export const logAppError=()=>{};
export const logAppInfo=()=>{};
export const appConfirm=async()=>false;
