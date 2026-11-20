#!/usr/bin/env python3
"""README 배너 생성기 → docs/images/banner.png (1280×420, 2배 크기로 그려 줄임).

필요: Pillow, 나눔 글꼴(fonts-nanum) 또는 --font-dir 로 NanumSquare*.ttf · NanumGothic*.ttf 가 든 폴더.
    python3 assets/make_banner.py
"""
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
FONT_DIRS = [Path(a) for a in sys.argv[1:]] + [Path("/usr/share/fonts/truetype/nanum")]
W, H, K = 1280, 420, 2


def font(name, size):
    for d in FONT_DIRS:
        if (d / name).exists():
            return ImageFont.truetype(str(d / name), size * K)
    raise SystemExit(f"글꼴 {name} 을 찾지 못했습니다")


def lerp(a, b, t):
    return tuple(round(x + (y - x) * t) for x, y in zip(a, b))


def main():
    img = Image.new("RGB", (W * K, H * K))
    d = ImageDraw.Draw(img)
    a, b = (0x0B, 0x1B, 0x4D), (0x1D, 0x4E, 0xD8)
    for x in range(W * K):
        d.line([(x, 0), (x, H * K)], fill=lerp(a, b, x / (W * K)))

    # 은은한 원 장식
    glow = Image.new("RGBA", img.size, (0, 0, 0, 0))
    g = ImageDraw.Draw(glow)
    for cx, cy, r, al in [(1120, 40, 260, 28), (980, 420, 200, 22), (80, 400, 160, 18)]:
        g.ellipse([(cx - r) * K, (cy - r) * K, (cx + r) * K, (cy + r) * K], fill=(255, 255, 255, al))
    img = Image.alpha_composite(img.convert("RGBA"), glow)
    d = ImageDraw.Draw(img)

    icon = Image.open(ROOT / "assets" / "icon.png").convert("RGBA").resize((168 * K, 168 * K), Image.LANCZOS)
    img.alpha_composite(icon, (84 * K, 126 * K))

    x = 292 * K
    d.text((x, 104 * K), "DOCX2HWPX-HLT", font=font("NanumSquareB.ttf", 64), fill=(255, 255, 255))
    d.text((x, 192 * K), "Word 특허 명세서를 특허청 전자출원 편집기 파일로", font=font("NanumGothicBold.ttf", 30), fill=(0xDB, 0xE7, 0xFF))

    # 형식 알약: DOCX → HLT · HWPX
    px, py = x, 262 * K
    for text, fill, ink in [
        ("DOCX", (255, 255, 255, 40), (255, 255, 255)),
        ("→", None, (0xA7, 0xF3, 0xD0)),
        ("HLT · NK-Editor", (0x10, 0xB9, 0x81, 255), (255, 255, 255)),
        ("HWPX · MOIP-Editor", (0x10, 0xB9, 0x81, 255), (255, 255, 255)),
    ]:
        f = font("NanumGothicBold.ttf", 22)
        tw = d.textlength(text, font=f)
        if fill is None:
            d.text((px, py + 10 * K), text, font=f, fill=ink)
            px += tw + 18 * K
            continue
        pill = Image.new("RGBA", img.size, (0, 0, 0, 0))
        ImageDraw.Draw(pill).rounded_rectangle([px, py, px + tw + 36 * K, py + 48 * K], radius=24 * K, fill=fill)
        img = Image.alpha_composite(img, pill)
        d = ImageDraw.Draw(img)
        d.text((px + 18 * K, py + 10 * K), text, font=f, fill=ink)
        px += tw + 36 * K + 14 * K

    out = ROOT / "docs" / "images" / "banner.png"
    out.parent.mkdir(parents=True, exist_ok=True)
    img.convert("RGB").resize((W, H), Image.LANCZOS).save(out, optimize=True)
    print(out.relative_to(ROOT))


if __name__ == "__main__":
    main()
