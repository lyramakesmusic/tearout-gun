import { readFileSync } from 'fs';
import { shellMidi } from '../dsp/snare.js';
const M = JSON.parse(readFileSync('out/gp2/t0.meta.json')), D = M.D, ids = M.knobs.map(k=>k.id), PB = M.pbins;
const P = new Float32Array(readFileSync('out/gp2/tt.params.f32').buffer.slice(0)), H = readFileSync('out/gp2/tt.pitch.u8');
const ix = (id)=>ids.indexOf(id), T = (i,id)=>{const k=M.knobs[ix(id)];return k.min+P[i*D+ix(id)]*(k.max-k.min)};
const fold=(c)=>{const d=((c%1200)+1200)%1200;return Math.min(d,1200-d)};
const res=[]; for (let i=0;i<P.length/D;i++){ const m=shellMidi(24+Math.round(T(i,'note_pc')),Math.round(T(i,'sn_t_oct')))+T(i,'sn_t_tune'), f=440*2**((m-69)/12);
  let kb=0; for(let k=1;k<PB;k++) if(H[i*PB+k]>H[i*PB+kb]) kb=k; const fp=70*2**(kb/36);
  res.push({lvl:T(i,'sn_t_lvl'), f, inRange: f>70&&f<2240, c:1200*Math.log2(fp/f), drop:T(i,'sn_t_pitch')}); }
const L=res.filter(r=>r.lvl>-2), med=a=>{a=[...a].sort((x,y)=>x-y);return a[a.length>>1]};
console.log('loud tone n',L.length,'f0 in feature range',L.filter(r=>r.inRange).length);
const R=L.filter(r=>r.inRange); console.log('argmax vs true f0: exact(<50ct)',(R.filter(r=>Math.abs(r.c)<50).length/R.length).toFixed(2),' pc-folded <50ct',(R.filter(r=>fold(r.c)<50).length/R.length).toFixed(2),' med |c|',med(R.map(r=>Math.abs(r.c))).toFixed(0));
console.log('f0 quantiles', [0.05,0.25,0.5,0.75,0.95].map(q=>med(L.map(r=>r.f)) && [...L.map(r=>r.f)].sort((a,b)=>a-b)[Math.floor(q*L.length)].toFixed(0)).join(' '));
