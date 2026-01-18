import { describe, it } from "jsr:@std/testing@1/bdd";
import { assertEquals } from "jsr:@std/assert@1";

// Simple test to verify testing framework works
describe('Test Setup Verification', () => {
  it('should pass a basic test', () => {
    assertEquals(1 + 1, 2);
  });

  it('should handle async operations', async () => {
    const result = await Promise.resolve(42);
    assertEquals(result, 42);
  });
});

describe('Counter Logic (Unit Tests)', () => {
  it('should increment count correctly', () => {
    let count = 0;
    count += 1;
    assertEquals(count, 1);
  });

  it('should handle multiple increments', () => {
    let count = 0;
    count += 1;
    count += 1;
    assertEquals(count, 2);
  });
});