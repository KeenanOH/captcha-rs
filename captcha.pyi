from __future__ import annotations


class Captcha(object):
    text: str
    base_img: str
    dark_mode: bool


class CaptchaBuilder(object):
    def __init__(self) -> CaptchaBuilder:
        ...

    def length(self, length: int) -> CaptchaBuilder:
        ...

    def width(self, width: int) -> CaptchaBuilder:
        ...

    def height(self, height: int) -> CaptchaBuilder:
        ...

    def dark_mode(self, dark_mode: bool) -> CaptchaBuilder:
        ...

    def complexity(self, complexity: int) -> CaptchaBuilder:
        ...

    def build(self) -> Captcha:
        ...
