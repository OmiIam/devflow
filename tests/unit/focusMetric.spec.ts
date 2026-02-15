import { describe, it, expect } from 'vitest';

type FocusState = 'active' | 'paused' | 'completed';

interface FocusSession {
  interruptions: number;
  state: FocusState;
  durationMinutes: number;
}

export function computeFocusScore(session: FocusSession) {
  if (session.durationMinutes <= 0) {
    throw new RangeError('duration must be positive');
  }
  const base = Math.min(session.durationMinutes / 60, 1);
  const penalty = Math.min(session.interruptions * 0.1, 0.5);
  const statePenalty = session.state === 'completed' ? 0 : 0.2;
  return Math.max(0, base - penalty - statePenalty);
}

const factory = (overrides: Partial<FocusSession> = {}): FocusSession => ({
  interruptions: 0,
  state: 'completed',
  durationMinutes: 25,
  ...overrides,
});

describe('computeFocusScore', () => {
  it('returns base score for uninterrupted completed session', () => {
    const score = computeFocusScore(factory());
    expect(score).toBeCloseTo(0.416, 3);
  });

  it('penalizes interruptions', () => {
    const score = computeFocusScore(factory({ interruptions: 3 }));
    expect(score).toBeLessThan(computeFocusScore(factory({ interruptions: 1 })));
  });

  it('penalizes non-completed states', () => {
    const paused = computeFocusScore(factory({ state: 'paused' }));
    const completed = computeFocusScore(factory({ state: 'completed' }));
    expect(paused).toBeLessThan(completed);
  });

  it('clamps penalty to zero', () => {
    const score = computeFocusScore(factory({ interruptions: 20 }));
    expect(score).toBeGreaterThanOrEqual(0);
  });

  it('rejects non-positive duration', () => {
    expect(() => computeFocusScore(factory({ durationMinutes: 0 }))).toThrow(
      RangeError,
    );
  });
});
