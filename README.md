# captcha-rs
[captcha-rs](https://github.com/samirdjelal/captcha-rs) implemented in Python.

## Install
```
pip install git+https://github.com/Shinobou/captcha-rs
```

## Examples
```py
from captcha_rs import Captcha, CaptchaBuilder

captcha: Captcha = CaptchaBuilder() \
    .length(5) \
    .width(130) \
    .height(50) \
    .dark_mode(False) \
    .complexity(5)
```
