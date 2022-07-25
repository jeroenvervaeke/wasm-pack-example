import init, { greet } from 'todos-rust';

(async () => {
    await init();
    greet();
})();