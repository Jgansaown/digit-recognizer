self.onmessage = function(msg: MessageEvent<number>) {
    const k = msg.data;
    const start = performance.now();
    const prime_list = _get_prime_list(k);
    const dur = performance.now() - start;
    self.postMessage({
        list: prime_list,
        dur: dur,
    });
}

function is_prime(n: number) {
    for (let i = 2; i < Math.trunc(Math.sqrt(n))+1; i++) {
        if (n % i == 0) {
            return false;
        }
    }
    return true;
}

function _get_prime_list(k: number): number[] {
    let ans = [];
    for (let i = 0; i < k; i++) {
        if (is_prime(i)) {
            ans.push(i);
        }
    }
    return ans;
}

export {};