import { describe, it, expect } from 'vitest';

function calculatePenalty(durationMinutes: number) {
  if (durationMinutes < 0) {
    throw new RangeError('duration must be non-negative');
  }
  if (durationMinutes === 0) {
    return 0;
  }
  return Math.min(1, durationMinutes / 60);
}

describe('calculatePenalty', () => {
  it('should return 0 for zero duration', () => {
    expect(calculatePenalty(0)).toBe(0);
  });

  it('should scale up for positive duration', () => {
    expect(calculatePenalty(15)).toBeCloseTo(0.25);
  });

  it('should clamp to 1 for long durations', () => {
    expect(calculatePenalty(120)).toBe(1);
  });

  it('should throw for negative duration', () => {
    expect(() => calculatePenalty(-5)).toThrow(RangeError);
  });
});
