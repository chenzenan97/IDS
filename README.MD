Project1
===================================================
This code uses the OpenCv library to read two images ,'foreground.jpg' and 'background.jpg ',and merge them together into a single image.


The code uses nested for loops to iterate through each pixel in the background image ,and for each pixel,it uses the alpha channel value of the corresponding pixel in the foreground image to calculate the blending of the two images. The alpha channel value is used to determine the transparency of the pixel,with as value of being fully transparents and value of 255 being fully opaque.

Project2
========================================
In this demo notebook we are going to demonstrate the creation and use of a ResNet model to see how the ResNet's skip connections can be implemented in code. The implementations demonstrated here are shallow ResNet model rather than the deep ResNet architectures described in the paper (and available in the PyTorch model zoo).
