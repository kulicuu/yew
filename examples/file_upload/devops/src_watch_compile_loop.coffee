c = console.log.bind console

util = require 'util'
exec = util.promisify((require 'child_process').exec)
chokidar = require 'chokidar'


COMMAND_ONE = 'wasm-pack build --target web --out-name wasm --out-dir ./static'


main = ->
    try
        { stdout, stderr } = await (exec COMMAND_ONE)
        c "stdout: #{stdout}"
        c "stderr: #{stderr}"
    catch err
        c err

main()

chokidar.watch('./src').on 'all', (event, path) ->
    c event, path
    try
        { stdout, stderr } = await (exec COMMAND_ONE)
        c "stdout: #{stdout}"
        c "stderr: #{stderr}"
    catch err
        c err