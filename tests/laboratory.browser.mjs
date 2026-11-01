import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {createServer} from 'vite';
import vue from '@vitejs/plugin-vue';
import {chromium} from 'playwright';
const root=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'..');
const server=await createServer({configFile:false,root,plugins:[vue()],logLevel:'error',resolve:{alias:[{find:'../services/scaleSimulator',replacement:path.join(root,'tests/fixtures/laboratory-api.ts')},{find:'../services/savedScaleModels',replacement:path.join(root,'tests/fixtures/saved-models-api.ts')}]},server:{host:'127.0.0.1',port:0}});
let browser;
try{
 await server.listen();browser=await chromium.launch({headless:true,args:['--no-sandbox','--disable-dev-shm-usage']});
 for(const width of [320,390,768,1366]){
  const page=await browser.newPage({viewport:{width,height:900}}),errors=[];page.on('pageerror',e=>errors.push(e.message));
  await page.goto(`http://127.0.0.1:${server.httpServer.address().port}/tests/fixtures/laboratory.html`);
  await page.locator('.preset-button').first().waitFor();
  assert.equal(await page.locator('.preset-button').count(),9);
  assert.equal(await page.locator('.scale-metric-card').count(),4);
  assert.equal(await page.locator('.preset-button').first().evaluate(e=>getComputedStyle(e).flexDirection),'column');
  assert.ok(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth+1),`simulation overflow ${width}`);
  const model=page.getByRole('combobox',{name:'Modelo salvo',exact:true});await model.locator('option[value=one]').waitFor({state:'attached'});await model.selectOption('one');assert.deepEqual(await page.evaluate(()=>window.__runtimeCalls),[]);
  await page.getByRole('button',{name:'Carregar',exact:true}).click();assert.deepEqual(await page.evaluate(()=>window.__runtimeCalls),[],'load model does not touch running instance');
  if(process.env.SCALE_UI_SCREENSHOTS){fs.mkdirSync(process.env.SCALE_UI_SCREENSHOTS,{recursive:true});await page.screenshot({path:path.join(process.env.SCALE_UI_SCREENSHOTS,`simulation-${width}.png`),fullPage:true});}
  await page.evaluate(()=>window.__go('/playground'));await page.locator('.playground-scale').waitFor();
  assert.ok(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth+1),`playground overflow ${width}`);
  await page.locator('.playground-launch select').selectOption('one');assert.deepEqual(await page.evaluate(()=>window.__runtimeCalls),[]);
  await page.getByRole('button',{name:'Iniciar instância',exact:true}).click();await page.waitForFunction(()=>window.__runtimeCalls.includes('start'));assert.equal(await page.locator('.playground-scale').count(),2);
  if(process.env.SCALE_UI_SCREENSHOTS)await page.screenshot({path:path.join(process.env.SCALE_UI_SCREENSHOTS,`playground-${width}.png`),fullPage:true});
  assert.deepEqual(errors,[]);console.log(JSON.stringify({width,realVuePages:true,syntheticRuntime:true,overflow:false,modelLoadPreservesInstances:true}));await page.close();
 }
}finally{if(browser)await browser.close();await server.close();}
