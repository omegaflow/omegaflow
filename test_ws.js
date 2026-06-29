const WebSocket = require('ws');

const ws = new WebSocket('ws://localhost:3571/pulse');
ws.binaryType = 'arraybuffer';

ws.on('open', () => {
    console.log('OPEN');
    
    // Build a binary frame exactly like world.js get() does
    const id = 1;
    const inputs = [
        { name: 'test.sensor', value: 42.0, t: 1000.0, x: 4000000, y: 500000, z: 4000000 }
    ];
    const queries = [
        { t: 1000.0, x: 4000000, y: 500000, z: 4000000 }
    ];
    
    let inputBytes = 0;
    for (const inp of inputs) inputBytes += 41 + inp.name.length;
    const buf = Buffer.alloc(8 + inputBytes + 4 + queries.length * 32);
    buf.writeUInt32LE(id, 0);
    buf.writeUInt32LE(inputs.length, 4);
    let off = 8;
    
    for (const inp of inputs) {
        buf.writeDoubleLE(inp.t, off); off += 8;
        buf.writeDoubleLE(inp.x, off); off += 8;
        buf.writeDoubleLE(inp.y, off); off += 8;
        buf.writeDoubleLE(inp.z, off); off += 8;
        buf.writeDoubleLE(inp.value, off); off += 8;
        buf.writeUInt8(inp.name.length, off); off += 1;
        Buffer.from(inp.name).copy(buf, off); off += inp.name.length;
    }
    
    buf.writeUInt32LE(queries.length, off); off += 4;
    for (const q of queries) {
        buf.writeDoubleLE(q.t, off); off += 8;
        buf.writeDoubleLE(q.x, off); off += 8;
        buf.writeDoubleLE(q.y, off); off += 8;
        buf.writeDoubleLE(q.z, off); off += 8;
    }
    
    console.log('Sending binary frame:', buf.length, 'bytes');
    ws.send(buf);
});

ws.on('message', (data, isBinary) => {
    console.log('MSG binary=' + isBinary, data.length, 'bytes');
    if (isBinary) {
        // Check φ header
        console.log('Header:', data[0], data[1], data[2]);
    } else {
        console.log('Text:', data.toString());
    }
});

ws.on('close', (code, reason) => {
    console.log('CLOSE code=' + code, reason.toString());
});

ws.on('error', (err) => {
    console.log('ERROR:', err.message);
});

setTimeout(() => { console.log('Done'); process.exit(0); }, 2000);
