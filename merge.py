import cv2  as cv 
def merge (foregound_path,background_path,f_posx,f_posy):
    f  = cv.imread(foregound_path)
    bg = cv.imread(background_path)
    if f_posy+f.shape[0] > bg.shape[0] or  f_posx+f.shape[1] > bg.shape[1]:
        raise ValueError('Out of bounds')
    for row in range(f_posy,f.shape[0]):
        for col in range(f_posx,f.shape[1]):
            f_alp = f[row,col][2]/255
            a = 1-f_alp
            bg[row,col] =(
                int(bg[row,col][0]*a + f[row,col][0]*f_alp),
                int(bg[row, col][1] * a + f[row, col][1] * f_alp),
                int(bg[row, col][2] * a + f[row, col][2] * f_alp)
            )
    cv.imwrite('merge.jpg',bg)
