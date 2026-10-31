from pathlib import Path
import zipfile,subprocess,hashlib,json
import argparse
args = argparse.ArgumentParser(description="Compare the real simulator encoders against real ADP parsers; no hardware.")
args.add_argument('--original-zip', required=True, type=Path)
args.add_argument('--adp-commands', required=True, type=Path)
args.add_argument('--out', required=True, type=Path)
args.add_argument('--rustc', default='rustc')
args = args.parse_args()
root=args.out.resolve(); root.mkdir(parents=True, exist_ok=True)
sim=Path(__file__).resolve().parents[1]; adp=args.adp_commands.resolve()
z=zipfile.ZipFile(args.original_zip)
old=z.read('src-tauri/src/commands/scale_simulator.rs').decode(); new=(sim/'src-tauri/src/commands/scale_simulator.rs').read_text(); parser=(adp/'scale_serial.rs').read_text()
(root/'original_simulator.rs').write_text(old);(root/'current_simulator.rs').write_text(new);(root/'scale_serial.rs').write_text(parser);(root/'toledo8540.rs').write_bytes((adp/'toledo8540.rs').read_bytes());(root/'toledo8540_encoder.rs').write_bytes((sim/'src-tauri/src/commands/toledo8540_encoder.rs').read_bytes())
pre='''#![allow(dead_code)]
#[path="toledo8540.rs"] mod toledo8540;
#[path="toledo8540_encoder.rs"] mod toledo8540_encoder;
#[derive(Clone,Copy,Debug)] enum ScaleSimulatorDriver {Wt1000n,Wt27r,Wt3000ir,Toledo8540,ToledoPrix3,A12e,LiderLd2052,Filizola,GenericAscii}
struct ScaleSimulatorConfig {driver:ScaleSimulatorDriver,unit:String,capacity_kg:f64,division_kg:f64}
struct ScaleState {gross_kg:f64,tare_kg:f64,stable:bool,gross_mode:bool}
'''
types=parser[parser.index('pub struct ScaleWeightData'):parser.index('#[derive(Debug, Clone, Serialize, Deserialize)]\npub struct ScaleStatusPayload')].replace('Utc::now().to_rfc3339()','String::new()')
pre+='#[derive(Debug)]\n'+types
func=parser[parser.index('fn parse_number('):parser.index('fn data_bits(')]
frame=new[new.index('fn build_frame('):new.index('fn status_from_runtime(')]
legacy=old[old.index('fn build_frame('):old.index('fn status_from_runtime(')].replace('fn build_frame(', 'fn legacy_build_frame(').replace('fn signed_value(', 'fn legacy_signed_value(').replace('signed_value(state','legacy_signed_value(state').replace('signed_value(net','legacy_signed_value(net')
main=r'''
fn main(){
 println!("version\tmodel\tcase\traw_hex\tframes\tgross\ttare\tnet\tstable\toverload\tunit\terrors");
 for (name,driver) in [("WT1000N",ScaleSimulatorDriver::Wt1000n),("WT27R",ScaleSimulatorDriver::Wt27r),("WT3000IR",ScaleSimulatorDriver::Wt3000ir),("TOLEDO8540",ScaleSimulatorDriver::Toledo8540),("TOLEDOPRIX3",ScaleSimulatorDriver::ToledoPrix3),("A12e",ScaleSimulatorDriver::A12e),("LIDERLD2052",ScaleSimulatorDriver::LiderLd2052),("FILIZOLA",ScaleSimulatorDriver::Filizola),("GENERICASCII",ScaleSimulatorDriver::GenericAscii)] {
  for (case,gross,tare,stable,gross_mode,unit,capacity,division) in [
   ("base",1234.0,0.0,true,true,"kg",80000.0,1.0),
   ("negative",-1234.0,0.0,true,true,"kg",80000.0,1.0),
   ("decimal",12.345,0.0,true,true,"kg",100.0,0.001),
   ("tare_gross",1234.0,200.0,true,true,"kg",80000.0,1.0),
   ("tare_net",1234.0,200.0,true,false,"kg",80000.0,1.0),
   ("negative_net",100.0,200.0,true,false,"kg",80000.0,1.0),
   ("unstable",1234.0,0.0,false,true,"kg",80000.0,1.0),
   ("overload",80001.0,0.0,true,true,"kg",80000.0,1.0),
   ("zero",0.0,0.0,true,true,"kg",80000.0,1.0),
   ("lb",4.5359237,0.90718474,true,false,"lb",100.0,0.01),
   ("grams",12.0,0.0,true,true,"g",100.0,1.0),
   ("wide",10000000.0,0.0,true,true,"kg",10000001.0,1.0)] {
    let c=ScaleSimulatorConfig{driver,unit:unit.into(),capacity_kg:capacity,division_kg:division};let s=ScaleState{gross_kg:gross,tare_kg:tare,stable,gross_mode};
    for (version,raw) in [("original",legacy_build_frame(&c,&s)),("current",build_frame(&c,&s))] {
     if name!="TOLEDO8540"{assert_eq!(legacy_build_frame(&c,&s),build_frame(&c,&s));}
     let mut buf=raw.as_bytes().to_vec(); let frames=split_equipment_frames(name,&mut buf);
     if !frames.is_empty(){let wire=[raw.as_bytes(),raw.as_bytes()].concat();for cut in 0..=wire.len(){let mut b=wire[..cut].to_vec();let mut fs=split_equipment_frames(name,&mut b);b.extend_from_slice(&wire[cut..]);fs.extend(split_equipment_frames(name,&mut b));assert_eq!(fs.len(),2,"framing {name} {case} {version} cut {cut}");}}
     let p=convert_frame(name,frames.first().map(Vec::as_slice).unwrap_or(raw.as_bytes()));
     println!("{version}\t{name}\t{case}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:?}\t{:?}",hex_encode(raw.as_bytes()),frames.len(),p.peso_bruto,p.tara,p.peso_liq,p.estavel,p.sobrecarga,p.unit,p.errors);
    }
  }
 }
}
'''
(root/'audit.rs').write_text(pre+func+frame+legacy+main)
rust=args.rustc
subprocess.run([rust,'--edition=2021',str(root/'audit.rs'),'-o',str(root/'audit')],check=True)
with (root/'results.tsv').open('w') as out: subprocess.run([str(root/'audit')],stdout=out,check=True)
print('Completed',len((root/'results.tsv').read_text().splitlines())-1,'cases; non-8540 frames unchanged; all emitted frame fragmentation checks passed.')
