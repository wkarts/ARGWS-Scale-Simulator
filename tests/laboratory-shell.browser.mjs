import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {createServer} from 'vite';
import vue from '@vitejs/plugin-vue';
import {chromium} from 'playwright';
const root=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'..');
const fixture=name=>path.join(root,'tests/fixtures',name);
const server=await createServer({configFile:false,root,plugins:[vue(),{name:'shell-service-fixtures',enforce:'pre',resolveId(source,importer){if(importer?.endsWith('/src/layouts/AppLayout.vue')&&['../services/crud','../services/logger','../services/tauri','../services/dialog'].includes(source))return fixture('shell-services.ts');}}],logLevel:'error',resolve:{alias:[{find:'../stores/session',replacement:fixture('shell-session.ts')},{find:'../services/scaleSimulator',replacement:fixture('laboratory-api.ts')},{find:'../services/savedScaleModels',replacement:fixture('saved-models-api.ts')}]},server:{host:'127.0.0.1',port:0}});
let browser;
try{
 await server.listen();browser=await chromium.launch({headless:true,args:['--no-sandbox','--disable-dev-shm-usage']});
 // 1093x558 reproduces the CSS area of a 1366x720 desktop at 125% scaling.
 for(const [width,height] of [[1366,698],[1093,558],[1024,640],[800,600],[390,844],[320,568]]){
  const page=await browser.newPage({viewport:{width,height}}),errors=[];page.on('pageerror',e=>errors.push(e.message));
  await page.route('**/*',r=>{const u=new URL(r.request().url());return u.port===String(server.httpServer.address().port)?r.continue():r.abort();});
  await page.goto(`http://127.0.0.1:${server.httpServer.address().port}/tests/fixtures/laboratory-shell.html`);
  await page.locator('.scale-simulator-page').waitFor();await page.evaluate(()=>document.fonts.ready);
  assert.equal(await page.locator('.professional-menu-links a.menu-item-current').count(),1,'only current route is highlighted');
  const overview=page.locator('.professional-menu-links a[href="/"]');
  assert.match(await overview.locator('xpath=ancestor::*[contains(@class,"menu-group")][1]').innerText(),/Principal/);
  assert.doesNotMatch(await overview.locator('xpath=ancestor::*[contains(@class,"menu-group")][1]').innerText(),/Laboratório/);
  assert.ok(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth+1),'shell must fit '+width);
  const hero=page.locator('.scale-simulator-page .dashboard-hero');
  assert.ok(await hero.locator('h2').evaluate(e=>parseFloat(getComputedStyle(e).fontSize))<=22,'compact heading');
  if(width>=1024)assert.ok((await hero.boundingBox()).height<=130,'short-window hero density');
  assert.ok(await page.locator('.scale-metric-card').first().evaluate(e=>e.getBoundingClientRect().height)<=72,'compact metrics');
  const screenshots=process.env.SCALE_UI_SCREENSHOTS;if(screenshots){fs.mkdirSync(screenshots,{recursive:true});await page.screenshot({path:path.join(screenshots,`shell-simulation-${width}x${height}.png`)});}
  const model=page.getByRole('combobox',{name:'Modelo salvo',exact:true});await model.locator('option[value=one]').waitFor({state:'attached'});await model.selectOption('one');await page.getByRole('button',{name:'Carregar',exact:true}).click();assert.deepEqual(await page.evaluate(()=>window.__runtimeCalls),[]);
  await page.evaluate(()=>window.__go('/playground'));await page.locator('.playground-scale').waitFor();
  assert.equal(await page.locator('.professional-menu-links a.menu-item-current').count(),1);
  assert.ok(await page.locator('.weight').first().evaluate(e=>parseFloat(getComputedStyle(e).fontSize))<=36,'weight does not scale with entire desktop width');
  if(width>=1300)assert.ok((await page.locator('.playground-scale').first().boundingBox()).width<650,'one instance does not stretch across entire workspace');
  assert.deepEqual(await page.locator('.playground-controls button').evaluateAll(xs=>xs.filter(e=>{const r=e.getBoundingClientRect();return r.width>0&&(r.left<0||r.right>innerWidth+1);}).map(e=>e.textContent)),[]);
  assert.deepEqual(await page.evaluate(()=>window.__runtimeCalls),[],'navigation does not change running instances');
  if(screenshots)await page.screenshot({path:path.join(screenshots,`shell-playground-${width}x${height}.png`)});
  await page.evaluate(()=>window.__go('/documentacao/guia'));await page.locator('.scale-guide-page').waitFor();assert.ok(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth+1));
  if(screenshots)await page.screenshot({path:path.join(screenshots,`shell-guide-${width}x${height}.png`)});
  assert.deepEqual(errors,[]);console.log(JSON.stringify({width,height,fullProductionShell:true,realVuePages:true,syntheticServices:true,densityChecked:true}));await page.close();
 }
}finally{if(browser)await browser.close();await server.close();}
