import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
const read=p=>fs.readFileSync(new URL('../'+p,import.meta.url),'utf8');
test('default theme retains the approved V2 palette and local font',()=>{const approved=JSON.parse(read('src/assets/branding/theme.json'));assert.deepEqual(approved.colors,{primary:'#102E49',accent:'#13B89D',secondaryText:'#65758A',background:'#F3F6FA',surface:'#FFFFFF',secondary:'#176B87'});assert.equal(approved.font,'DejaVu Sans');assert.equal(approved.defaultMode,'light');const theme=read('src/visual-core/theme.ts');assert.match(theme,/accent: approvedScaleTheme.colors.accent/);assert.match(theme,/preset: "scale-signal"/);assert.match(theme,/fontFamily: "ScaleSans, Segoe UI, Arial, sans-serif"/);const css=read('src/styles/scale-laboratory.css');for(const color of Object.values(approved.colors))assert(css.includes(color));assert.match(css,/data-theme="light"/);});
