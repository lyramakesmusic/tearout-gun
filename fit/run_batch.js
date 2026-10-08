// Run a fit script over a job list (tab-separated: path, id) with N workers.
// usage: node fit/run_batch.js <script> <jobs.tsv> <outdir> <prefix> [workers] [scale]
import { spawn } from 'child_process';
import { readFileSync, existsSync, appendFileSync } from 'fs';
const [script, jobsFile, outdir, prefix, W = '12', scale = '1'] = process.argv.slice(2);
const jobs = readFileSync(jobsFile, 'utf8').trim().split('\n').map((l) => l.split('\t')).filter(([, id]) => !existsSync(`${outdir}/${prefix}${id}.json`));
let next = 0, done = 0; const t0 = Date.now();
const run = () => {
  if (next >= jobs.length) return;
  const [path, id] = jobs[next++];
  const p = spawn('nice', ['-n', '18', 'node', script, path, `${outdir}/${prefix}${id}.json`, scale], { stdio: ['ignore', 'ignore', 'pipe'] });
  let err = ''; p.stderr.on('data', (d) => (err += d));
  p.on('close', (code) => { appendFileSync(`${outdir}/log.txt`, `[${id}] exit ${code}\n${err}`); done++; console.log(`${done}/${jobs.length} ${((Date.now() - t0) / 60000).toFixed(1)} min`); run(); });
};
for (let i = 0; i < +W; i++) run();
