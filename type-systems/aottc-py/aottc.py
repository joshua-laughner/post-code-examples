from typing import Sequence, Any


def read_rh() -> Sequence[float]:
    # pretend we read this from a sensor or file
    return [96.0, 95.0]


def humidity_demo():
    humidity = read_rh(True)
    print(humidity)


def calc_pot_temp(t: float, p: float) -> float:
    return t * (1000.0/p)**0.286


def demo(p: Any) -> None:
    calc_pot_temp(1, p)
