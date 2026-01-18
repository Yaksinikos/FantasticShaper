import { describe, it, expect } from 'vitest';

// Simple test to verify testing framework works
describe('Test Setup Verification', () => {
  it('should pass a basic test', () => {
    expect(1 + 1).toBe(2);
  });

  it('should handle async operations', async () => {
    const result = await Promise.resolve(42);
    expect(result).toBe(42);
  });
});

describe('Counter Logic (Unit Tests)', () => {
  it('should increment count correctly', () => {
    let count = 0;
    count += 1;
    expect(count).toBe(1);
  });

  it('should handle multiple increments', () => {
    let count = 0;
    count += 1;
    count += 1;
    expect(count).toBe(2);
  });
});