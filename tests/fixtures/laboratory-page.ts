import {createApp,h} from 'vue';
import {createRouter,createMemoryHistory,RouterView} from 'vue-router';
import Simulation from '../../src/pages/ScaleSimulatorPage.vue';
import Playground from '../../src/pages/PlaygroundPage.vue';
import '../../src/styles.css';
import '../../src/styles/scale-laboratory.css';
const router=createRouter({history:createMemoryHistory(),routes:[{path:'/',component:Simulation},{path:'/playground',component:Playground}]});
const app=createApp({render:()=>h(RouterView)});app.use(router);router.isReady().then(()=>app.mount('#app'));(window as any).__go=(route:string)=>router.push(route);
