import assert from 'node:assert/strict';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {createServer} from 'vite';
import vue from '@vitejs/plugin-vue';
import {chromium} from 'playwright';
const root=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'..');
const server=await createServer({configFile:false,root,plugins:[vue()],logLevel:'error',resolve:{alias:[{find:'../services/savedScaleModels',replacement:path.join(root,'tests/fixtures/saved-models-api.ts')}]},server:{host:'127.0.0.1',port:0}});
let browser;
try{
 await server.listen();const port=server.httpServer.address().port;
 browser=await chromium.launch({headless:true,...(process.env.BROWSER_EXECUTABLE?{executablePath:process.env.BROWSER_EXECUTABLE}:{}),args:['--no-sandbox','--disable-dev-shm-usage']});
 for(const width of [320,360,768,1366]){
  const page=await browser.newPage({viewport:{width,height:800}});const errors=[];page.on('pageerror',e=>errors.push(e.message));
  await page.goto(`http://127.0.0.1:${port}/tests/fixtures/saved-models.html`);
  const select=page.getByRole('combobox',{name:'Modelo salvo',exact:true});await select.locator('option[value=one]').waitFor({state:'attached'});
  const size=await page.locator('.saved-models').boundingBox();assert.ok(size.height<=90,`compact toolbar at ${width}: ${size.height}`);
  assert.ok(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth),'no horizontal page overflow');
  await select.selectOption('two');assert.equal(await page.evaluate(()=>window.__loadedModel??null),null,'selection alone never loads or starts');
  await select.selectOption('one');await page.getByRole('button',{name:'Carregar',exact:true}).click();
  assert.equal(await page.evaluate(()=>window.__loadedModel.serial_port),'COM10');assert.equal(await page.evaluate(()=>window.__loadedModel.id),null);
  await page.getByRole('button',{name:'Salvar…',exact:true}).click();await page.getByLabel('Nome do modelo',{exact:true}).fill('Modelo responsivo '+width);
  const modal=await page.locator('.app-modal-card').boundingBox();assert.ok(modal.x>=0&&modal.x+modal.width<=width+1,'modal fits viewport');
  await page.getByRole('button',{name:'Salvar como novo',exact:true}).click();await page.locator('.app-modal-card').waitFor({state:'detached'});
  await page.getByRole('button',{name:'Gerenciar modelos salvos',exact:true}).click();await page.getByRole('button',{name:'Remover',exact:true}).click();
  assert.equal(await page.evaluate(()=>window.__modelCalls.filter(x=>x==='delete').length),0,'removal requires confirmation');
  await page.getByRole('button',{name:'Confirmar remoção',exact:true}).click();
  assert.equal(await page.evaluate(()=>window.__modelCalls.filter(x=>x==='delete').length),1);
  assert.deepEqual(await page.evaluate(()=>window.__activeInstances),[{id:'existing-instance',running:true,driver:'generic_ascii'}]);
  assert.deepEqual(errors,[]);console.log(JSON.stringify({width,toolbarHeight:size.height,modalWidth:modal.width,result:'PASS',fixture:'actual Vue component, synthetic model API'}));await page.close();
 }
}finally{if(browser)await browser.close();await server.close();}
