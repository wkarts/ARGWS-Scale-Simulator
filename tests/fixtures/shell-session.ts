import {reactive} from 'vue';
const session=reactive({user:{id:1,login:'qa-local',nome:'Operador de teste',company_ids:[],company_names:[]},sessionToken:'',activeCompanyId:null,activeCompanyName:'Bancada de teste',isMaster:true,can:()=>true,setActiveCompany:()=>{},validateCurrentSession:async()=>true,logout:async()=>{}});
export const useSessionStore=()=>session;
