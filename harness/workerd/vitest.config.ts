import { cloudflareTest } from "@cloudflare/vitest-pool-workers";
import { defineConfig } from "vitest/config";

export default defineConfig({
  plugins: [
    cloudflareTest({
      wrangler: { configPath: "./wrangler.toml" },
    }),
  ],
  test: {
    // A rule that panics traps the wasm instance; the shim reinitialises it on
    // the next call, so rules run one at a time and never share an instance
    // mid-flight.
    fileParallelism: false,
    testTimeout: 60_000,
  },
});
