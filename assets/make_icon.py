#!/usr/bin/env python3
"""앱 아이콘 생성기 → assets/icon.png(512) · assets/icon.ico(16~256).

파란 둥근 사각형 위에 흰 명세서 한 장, 오른쪽 아래에 「바꾸기」 화살표 배지.
1024px 로 그린 뒤 줄여서 가장자리를 매끄럽게 한다.  필요: Pillow.
    python3 assets/make_icon.py
"""
from pathlib import Path

from PIL import Image, ImageDraw

S = 1024
HERE = Path(__file__).resolve().parent


def lerp(a, b, t):
    return tuple(round(x + (y - x) * t) for x, y in zip(a, b))


def draw() -> Image.Image:
    img = Image.new("RGBA", (S, S), (0, 0, 0, 0))

    # 1) 바탕: 위→아래 그라데이션 둥근 사각형
    grad = Image.new("RGBA", (S, S))
    top, bottom = (0x3B, 0x82, 0xF6), (0x1D, 0x3F, 0xB8)
    gd = ImageDraw.Draw(grad)
    for y in range(S):
        gd.line([(0, y), (S, y)], fill=lerp(top, bottom, y / S) + (255,))
    mask = Image.new("L", (S, S), 0)
    ImageDraw.Draw(mask).rounded_rectangle([40, 40, S - 40, S - 40], radius=220, fill=255)
    img.paste(grad, (0, 0), mask)

    d = ImageDraw.Draw(img)

    # 2) 명세서 한 장(접힌 귀퉁이)
    x0, y0, x1, y1, fold = 250, 190, 720, 820, 150
    shadow = Image.new("RGBA", (S, S), (0, 0, 0, 0))
    ImageDraw.Draw(shadow).polygon(
        [(x0 + 14, y0 + 22), (x1 - fold + 14, y0 + 22), (x1 + 14, y0 + fold + 22), (x1 + 14, y1 + 22), (x0 + 14, y1 + 22)],
        fill=(10, 30, 90, 70),
    )
    img.alpha_composite(shadow)
    d.polygon([(x0, y0), (x1 - fold, y0), (x1, y0 + fold), (x1, y1), (x0, y1)], fill=(255, 255, 255, 255))
    d.polygon([(x1 - fold, y0), (x1 - fold, y0 + fold), (x1, y0 + fold)], fill=(0xC7, 0xD7, 0xFE, 255))

    # 본문 줄: 제목 한 줄 + 문단 줄
    d.rounded_rectangle([x0 + 60, y0 + 80, x0 + 250, y0 + 120], radius=20, fill=(0x25, 0x5F, 0xD6, 255))
    for i, w in enumerate([340, 300, 340, 220, 300]):
        yy = y0 + 200 + i * 70
        d.rounded_rectangle([x0 + 60, yy, x0 + 60 + w, yy + 28], radius=14, fill=(0xCB, 0xD5, 0xE1, 255))

    # 3) 배지: 초록 원 + 오른쪽 화살표
    cx, cy, r = 735, 735, 175
    d.ellipse([cx - r - 18, cy - r - 18, cx + r + 18, cy + r + 18], fill=(255, 255, 255, 255))
    d.ellipse([cx - r, cy - r, cx + r, cy + r], fill=(0x10, 0xB9, 0x81, 255))
    d.rounded_rectangle([cx - 95, cy - 22, cx + 40, cy + 22], radius=22, fill=(255, 255, 255, 255))
    d.polygon([(cx + 10, cy - 80), (cx + 105, cy), (cx + 10, cy + 80)], fill=(255, 255, 255, 255))
    return img


def main():
    big = draw()
    big.resize((512, 512), Image.LANCZOS).save(HERE / "icon.png")
    sizes = [(16, 16), (24, 24), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]
    big.resize((256, 256), Image.LANCZOS).save(HERE / "icon.ico", sizes=sizes)
    print("assets/icon.png, assets/icon.ico")


if __name__ == "__main__":
    main()
