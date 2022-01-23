const gulp = require('gulp')
const path = require('path')
const {spawn} = require('child_process')
const workspaceDir = path.resolve(__dirname, '../..')

const rustSrc = [
    path.resolve(workspaceDir, 'crates/game/src/**/*.rs')
]
const wasmPackDirs = [
    path.resolve(workspaceDir, 'crates/game')
]

const wasmPackArgs = ['--dev']

function wasmPack(dir) {
    return (cb) => {
        const ps = spawn('wasm-pack', ['build', '-t', 'web'].concat(wasmPackArgs), {
            stdio: 'inherit',
            cwd: dir
        })
        ps.on('error', (err) => {
            cb(err)
        })

        ps.on('exit', (rc) => {
            if (!!rc) {
                cb(new Error(`wasm-pack returned nonzero exit code ${rc}`))
            } else {
                cb()
            }
        })
    }
}

function vite(cb) {
    ps = spawn('yarn', ['dev'], {
        stdio: 'inherit'
    })
    ps.on('exit', (rc) => {
        if (!!rc) {
            cb(new Error(`vite returned nonzero exit code ${rc}`))
        } else {
            cb()
        }
    })
}

const wasmPackGame = wasmPack(wasmPackDirs[0])
const wasmPackGameWatch = gulp.series(wasmPackGame, () => {
    return gulp.watch([rustSrc[0]], wasmPackGame)
})

exports.vite = vite
exports.wasmPackGame = wasmPackGame
exports.wasmPackGameWatch = wasmPackGameWatch
exports.default = gulp.parallel(wasmPackGameWatch)