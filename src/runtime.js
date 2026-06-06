/**
 * Runtime initialization for deno_runner
 *
 * This module sets up the JavaScript runtime environment with:
 * - Console logging functionality
 * - Global registration of Rust operations
 * - Helper functions for calling Rust code
 */
;((globalThis) => {
  'use strict'

  const core = Deno.core

  /**
   * Converts arguments to a formatted message string
   * @param {...any} args - Arguments to format
   * @returns {string} Formatted message
   */
  function argsToMessage(...args) {
    return args
      .map((arg) => {
        try {
          if (typeof arg === 'string') {
            return arg
          }
          return JSON.stringify(arg, null, 2)
        } catch (error) {
          // Handle circular references and other serialization errors
          return String(arg)
        }
      })
      .join(' ')
  }

  /**
   * Enhanced console implementation with better error handling
   */
  globalThis.console = {
    log: (...args) => {
      try {
        core.print(`[out]: ${argsToMessage(...args)}\n`, false)
      } catch (error) {
        core.print(`[out]: <error formatting message>\n`, false)
      }
    },

    error: (...args) => {
      try {
        core.print(`[err]: ${argsToMessage(...args)}\n`, true)
      } catch (error) {
        core.print(`[err]: <error formatting message>\n`, true)
      }
    },

    warn: (...args) => {
      try {
        core.print(`[warn]: ${argsToMessage(...args)}\n`, true)
      } catch (error) {
        core.print(`[warn]: <error formatting message>\n`, true)
      }
    },

    info: (...args) => {
      try {
        core.print(`[info]: ${argsToMessage(...args)}\n`, false)
      } catch (error) {
        core.print(`[info]: <error formatting message>\n`, false)
      }
    },

    debug: (...args) => {
      try {
        core.print(`[debug]: ${argsToMessage(...args)}\n`, false)
      } catch (error) {
        core.print(`[debug]: <error formatting message>\n`, false)
      }
    },
  }

  /**
   * Register all Rust operations as global functions
   * This allows calling Rust functions directly from JavaScript
   */
  if (core.ops) {
    for (const opName of Object.keys(core.ops)) {
      try {
        const op = core.ops[opName]
        // Create a wrapper function that calls the operation
        globalThis[opName] = (...args) => {
          return op(...args)
        }
      } catch (error) {
        console.error(`Failed to register operation: ${opName}`, error)
      }
    }
  }

  /**
   * Helper function to call Rust operations synchronously
   * @param {string} opName - Name of the operation
   * @param {...any} args - Arguments to pass to the operation
   * @returns {any} Result from the Rust operation
   */
  globalThis.rust = (opName, ...args) => {
    if (!core.ops[opName]) {
      throw new Error(`Unknown Rust operation: ${opName}`)
    }
    return core.ops[opName](...args)
  }

  /**
   * Helper function to call Rust operations asynchronously
   * @param {string} opName - Name of the operation
   * @param {...any} args - Arguments to pass to the operation
   * @returns {Promise<any>} Promise resolving to the result
   */
  globalThis.rustAsync = async (opName, ...args) => {
    if (!core.ops[opName]) {
      throw new Error(`Unknown Rust operation: ${opName}`)
    }
    return await core.ops[opName](...args)
  }
})(globalThis)
