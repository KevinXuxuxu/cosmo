import init, { PlayerWASM } from '/static/wasm/cosmo/cosmo.js';
async function run() {
    await init();
    let scene = [
        'L P 15 15 15 400 -',
        'C P -1 -1 0 30 30 0 60 2',
        'P A 0 0 8.660254',
        'P B 0 0 -8.660254',
        'P C 8.164965 0 2.886751',
        'P D -4.082483 7.071067 2.886751',
        'P E -4.082483 -7.071067 2.886751',
        'P F 4.082483 7.071067 -2.886751',
        'P G -8.164965 0 -2.886751',
        'P H 4.082483 -7.071067 -2.886751',
        'OBJ',
        'T A C D - ',
        'T C F D -',
        'T A D E *',
        'T D G E *',
        'T A E C .',
        'T E H C .',
        'T D F G #',
        'T F B G #',
        'T C H F /',
        'T H B F /',
        'T E G H @',
        'T G B H @',
        'M R 90 0 0 0 0 0 1',
        'END_OBJ'
    ];
    window.cosmo_player = PlayerWASM.new(scene, 24, 60, 40, false, false, false);
    window.PlayerWASM = PlayerWASM;
    window.cosmo_run = function (duration, cb) {
        const intervalId = setInterval(() => {
            cosmo_player.update();
            cb(cosmo_player.get_a().join('\n'));
        }, 1000.0/24.0)
        setTimeout(() => {
            clearInterval(intervalId);
        }, duration * 1000);
    };
}
run();