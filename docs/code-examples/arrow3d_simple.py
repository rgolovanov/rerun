"""Log a single arrow."""
import rerun as rr

rr.init("arrow", spawn=True)

rr.log_arrow("simple", origin=[0, 0, 0], vector=[1, 0, 1], width_scale=0.05)