import sys
import time

TOTAL_DURATION = 60  # Total duration of the progress bar in seconds
STEPS = 50  # Number of steps in the progress bar

def main():
    interval = TOTAL_DURATION / STEPS
    for step in range(STEPS + 1):
        progress = "=" * step + " " * (STEPS - step)
        percentage = step * 100 // STEPS
        remaining_time = (STEPS - step) * interval
        eta_minutes = int(remaining_time // 60)
        eta_seconds = int(remaining_time % 60)
        sys.stdout.write(f"\r[{progress}] {percentage:3}% (ETA: {eta_minutes:02}:{eta_seconds:02})")
        sys.stdout.flush()
        time.sleep(interval)

    print("\nDone!")

if __name__ == "__main__":
    main()
