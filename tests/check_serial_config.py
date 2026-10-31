"""Compile the complete serial simulator without Tauri attributes or OS port discovery.
No serial port is opened. A TCP loopback test uses an ephemeral 127.0.0.1 port.
Uses the real serialport builder, serde config, and simulator source.
Run: python tests/check_serial_config.py [--cargo PATH] [--offline]
"""
import argparse
import json
import pathlib
import subprocess
import tempfile

p = argparse.ArgumentParser()
p.add_argument('--cargo', default='cargo')
p.add_argument('--offline', action='store_true')
a = p.parse_args()
root = pathlib.Path(__file__).resolve().parents[1]
source = (root/'src-tauri/src/commands/scale_simulator.rs').read_text()
source = source.replace('#[tauri::command]\n', '')
source = source.replace('#[path = "toledo8540_encoder.rs"]', '#[path = '+json.dumps(str(root/'src-tauri/src/commands/toledo8540_encoder.rs'))+']')
source = source.replace('#[path = "frame_test_overrides.rs"]', '#[path = '+json.dumps(str(root/'src-tauri/src/commands/frame_test_overrides.rs'))+']')
source += '\n' + (root/'tests/check_serial_config.rs').read_text()
manifest = '''[package]
name="simulator-serial-parameters-check"
version="0.1.0"
edition="2021"
[dependencies]
serialport={version="4.6",default-features=false}
serde={version="1",features=["derive"]}
serde_json="1"
chrono={version="0.4",features=["serde"]}
uuid={version="1",features=["v4","serde"]}
tokio={version="1",features=["rt-multi-thread","macros","net","signal","io-util","time"]}
'''
with tempfile.TemporaryDirectory(prefix='simulator-serial-') as temp:
    temp=pathlib.Path(temp)
    (temp/'src').mkdir()
    (temp/'Cargo.toml').write_text(manifest)
    (temp/'src/lib.rs').write_text(source)
    subprocess.run([a.cargo,'test','--manifest-path',str(temp/'Cargo.toml')]+(['--offline'] if a.offline else []),check=True)
