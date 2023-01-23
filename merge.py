import cv2 
def merge (f,bg,f_posix,f_posiy):
    if f_posiy+f.shape[0] > bg.shape[0] or  f_posix+f.shape[1] > bg.shape[1]:
        raise ValueError('Out of bounds')
    for row in range(f_posiy,f.shape[0]):
        for col in range(f_posix,f.shape[1]):
            f_alp = f[row,col][2]/255
            a = 1-f_alp
            bg[row,col] =(
                int(bg[row,col][0]*a + f[row,col][0]*f_alp),
                int(bg[row, col][1] * a + f[row, col][1] * f_alp),
                int(bg[row, col][2] * a + f[row, col][2] * f_alp)
            )
    cv.imwrite('merge.jpg',bg)
