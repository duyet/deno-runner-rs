;((globalThis) => {
  const core = Deno.core

  function argsToMessage(...args) {
    return args.map((arg) => JSON.stringify(arg)).join(' ')
  }

  globalThis.console = {
    log: (...args) => {
      core.print(`[out]: ${argsToMessage(...args)}\n`, false)
    },
    error: (...args) => {
      core.print(`[err]: ${argsToMessage(...args)}\n`, true)
    },
  }

  // Re-export op to `globalThis`
  for (let op of Object.keys(core.ops)) {
    globalThis[op] = (...args) => {
      return core.opSync(op, ...args)
    }
  }

  // Re-export opSync and opAsync to `globalThis`
  // Usage: rust("op_name", arg1, arg2, ...)
  globalThis.rust = core.opSync
  globalThis.rustAsync = core.opAsync
})(globalThis)
