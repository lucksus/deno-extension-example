((globalThis) => {
    const core = Deno.core;

    globalThis.EXT = {
        sum: (data) => {
            return core.opAsync("op_sum", data);
        },
    };
  })(globalThis);
  