
import ezdxf


def main(filename):
    doc = ezdxf.readfile(filename)
    msp = doc.modelspace()
    lines = msp.query('LINE')

    points = [p for line in lines for p in (line.dxf.start, line.dxf.end)]
    x, y, z = zip(*points)

    print('min_x =', min(x))
    print('max_x =', max(x))
    print('min_y =', min(y))
    print('max_y =', max(y))


if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument('filename')
    main(**vars(parser.parse_args()))
