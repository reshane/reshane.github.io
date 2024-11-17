use yew::prelude::*;
use std::collections::HashMap;
pub struct Posts { pub posts: HashMap::<String, Html> }
impl Posts {
    pub fn new() -> Self {
        let mut post_map = HashMap::<String, Html>::new();
    	post_map.insert(String::from("20241113_convolution"), html! {<span markdown="block" style="white-space: pre-wrap"><div markdown="span">
<h1>{ r#"Convolution"# }</h1>
<div>{ r#"I want to do some convolution stuff in c because it seems fun."# }</div>
<div>{ r#"So I start by making a project and taking some screenshots for this blog I guess."# }</div>
</div>
<img src={"/build/20241113_convolution/images/0_file_structure.png"}/>
<div>
</div>
<img src={"/build/20241113_convolution/images/1_initial_main_c.png"}/>
<div>
</div>
<img src={"/build/20241113_convolution/images/2_initial_convolve_h.png"}/>
<div>
</div>
<img src={"/build/20241113_convolution/images/3_initial_convolve_c.png"}/>
<div>
<div>{ r#"Now to figure out how to build all of it..."# }</div>
<div>{ r#""# }</div><a href={ "https://stackoverflow.com/questions/1705961/how-to-link-to-a-static-library-in-c" }>{ "To stack overflow!" }
</a><div>{ r#""# }</div>
<div>{ r#"And we end up with this:"# }</div>
</div>
<img src={"/build/20241113_convolution/images/4_build_script_initial.png"}/>
<div>
<div>{ r#"Which allows us to do this: "# }</div>
</div><video autoplay=true width=500 loop=true>
<source src={"/build/20241113_convolution/images/5_initial_run.webm"} type="video/webm"/>
</video><div>
<div>{ r#"Great! Now let's warm up with some light convolution before starting the crazy stuff..."# }</div>
<div>{ r#"For that I'm going to write a data structure to help out a little"# }</div>
</div>
<img src={"/build/20241113_convolution/images/6_int_vector.png"}/>
<div>
<div>{ r#"Admitedly, the naming of this thing is not great, but refactoring is a beautiful thing that I expect to do a lot of later."# }</div>
<div>{ r#"For now, we can just start implementing this."# }</div>
<div>{ r#"Most of it is pretty simple, malloc for our structure once and then malloc/realloc for our data once the capacity is exceeded..."# }</div>
<pre><code>{{ r#"
// vector

Vector_i* Vector_i_new() {
    Vector_i* new = (Vector_i*) malloc(sizeof(Vector_i));
    if (!new) {
        printf("ERROR: Could initialize vector!");
        exit(1);
    }
    new->size = 0;
    new->capacity = 0;
    new->data = NULL;
    return new;
}

void Vector_i_push(Vector_i* v, int e) {
    if (v->capacity == 0) {
        v->capacity = 1;
        v->data = (int*) malloc(v->capacity * sizeof(int));
        if (!v->data) exit(1);
    }
    else if (v->size == v->capacity) {
        v->capacity *= 2;
        int* newMem = (int*) realloc(v->data, v->capacity * sizeof(int));
        if (!newMem) {
            // we have an error here... uh-oh!
            printf("ERROR: Could not increase the size of the vector");
            exit(1);
        }
        v->data = newMem;
    }
    v->data[v->size++] = e;
}

int Vector_i_get(Vector_i* v, size_t idx) {
    assert(idx < v->size);
    return v->data[idx];
}

size_t Vector_i_size(Vector_i* v) {
    return v->size;
}

size_t Vector_i_capacity(Vector_i* v) {
    return v->capacity;
}

void Vector_i_free(Vector_i* v) {
    free(v->data);
    free(v);
}
"# }}</code></pre>
<div>{ r#"Also important to implement a free function so we aren't leaking a bunch of memory."# }</div>
<div>{ r#"Even though what I'm going to be using this for will pretty much exclusivly be short-lived programs, just in case I ever use it for something longer, it'll be nice to have this function."# }</div>
<div>{ r#"And best practices and all that stuff of course."# }</div>
<div>{ r#"We can also write a test for it"# }</div>
<div>{ r#""# }</div><a href={ "https://www.youtube.com/watch?v=5aZiRjgSGQU" }>{ "Thanks to Kay Lack for the little test framework" }
</a><div>{ r#""# }</div>
</div>
<img src={"/build/20241113_convolution/images/8_tests_initial.png"}/>
<div>
<div>{ r#"(ignore all that "# }<code>{ r#"Vector_f"# }</code>{ r#" stuff, that's for later and I forgot to remove it for the screenshot...)"# }</div>
<div>{ r#"Now for the "# }<code>{ r#"Vector_i_convolve"# }</code>{ r#" function..."# }</div>
<div>{ r#"I'll want to take in two vectors to perform the convolution over eachother, so we'll call them a and b"# }</div>
<div>{ r#"And basically what we'll be doing is something like this:"# }</div>
<pre><code>{{ r#"
                           a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
                        a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
                     a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
                  a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
               a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
            a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
         a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
      a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
   a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
   b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
      b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
         b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
            b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
               b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
                  b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
                     b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
                        b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
                           b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
"# }}</code></pre>
<div>{ r#"where we can imagine flipping b around so that its lowest indices are on the right,"# }</div>
<div>{ r#"and its 0th index is placed under the 0th index of a"# }</div>
<div>{ r#"we can then multuply the elements that are 'facing eachother', and shift b to the right"# }</div>
<div>{ r#"Now, dealing with the pairs of operations should look something like this"# }</div>
<pre><code>{{ r#"
a[0] * b[0]
a[0] * b[1] + a[1] * b[0]
a[0] * b[2] + a[1] * b[1] + a[2] * b[0]
...
a[1] * b[9] + a[2] * b[8] * a[3] * b[7] ... 
a[2] * b[9] + a[3] * b[8] * a[4] * b[7] ... 
"# }}</code></pre>
<div>{ r#"And after lots of thought and effort, we can write this function:"# }</div>
<pre><code>{{ r#"
Vector_i* Vector_i_convolve(Vector_i* a, Vector_i* b) {
    Vector_i* result = Vector_i_new();

    int a_size = Vector_i_size(a);
    int b_size = Vector_i_size(b);

    int a_start = 0, b_start = 0;
    for (int i=0; i<a_size+b_size; ++i) {
        int j=a_start,k=b_start,total=0;
        while (j<a_size && k>-1) {
            total += Vector_i_get(a, j) * Vector_i_get(b, k);
            j++;
            k--;
        }
        Vector_i_push(result, total);
        if (b_start == b_size-1) {
            a_start++;
        } else {
            b_start++;
        }
    }

    return result;
}
"# }}</code></pre>
<div>{ r#"and then we can try it out in our main:"# }</div>
<div>{ r#"(Note that I have DEBUG in a "# }<code>{ r#"#define"# }</code>{ r#" statement above main to turn the printing on/off"# }</div>
<pre><code>{{ r#"
int main() {
    Vector_i* base = Vector_i_new();
    Vector_i* mask = Vector_i_new();

    for (int i = 0; i < SIZE; ++i) {
        Vector_i_push(base, i);
        if (DEBUG) printf("vector contents\n\tsize: %d\n\tcap: %d\n\tlast: %d\n", base->size, base->capacity, base->data[base->size-1]);
        Vector_i_push(mask, i);
        if (DEBUG) printf("vector contents\n\tsize: %d\n\tcap: %d\n\tlast: %d\n", mask->size, mask->capacity, mask->data[mask->size-1]);
    }

    if (DEBUG) printf("vector contents\n\tsize: %d\n\tcap: %d\n\tlast: %d\n", base->size, base->capacity, base->data[base->size-1]);
    if (DEBUG) printf("vector contents\n\tsize: %d\n\tcap: %d\n\tlast: %d\n", mask->size, mask->capacity, mask->data[mask->size-1]);

    Vector_i* result = Vector_i_convolve(base, mask);

    if (1) {
        printf("[");
        for (int i = 0; i < Vector_i_size(result); ++i) {
            printf("%d", Vector_i_get(result, i));
            if (i+1 != Vector_i_size(result)) printf(",");
        }
        printf("]");
    }

    Vector_i_free(base);
    Vector_i_free(mask);
    Vector_i_free(result);

    return 0;
}
"# }}</code></pre>
<div>{ r#"And with "# }<code>{ r#"DEBUG = 0"# }</code>{ r#", when we "# }<code>{ r#"./build.sh && ./build/main"# }</code>{ r#""# }</div>
</div>
<img src={"/build/20241113_convolution/images/7_vector_out_initial.png"}/>
<div>
<div>{ r#"Great! Thats probably right... "# }</div>
<div>{ r#"Lets start in the convolve.h file by defining our interface:"# }</div>
<pre><code>{{ r#"
// matrix

typedef struct {
    size_t rows;
    size_t cols;
    int* data;
} Matrix_i;

Matrix_i* Matrix_i_new(size_t rows, size_t cols);

size_t Matrix_i_rows(Matrix_i* m);

size_t Matrix_i_cols(Matrix_i* m);

void Matrix_i_set(Matrix_i* m, size_t x, size_t y, int e);

int Matrix_i_get(Matrix_i* m, size_t x, size_t y);

void Matrix_i_free(Matrix_i* m);

Matrix_i* Matrix_i_convolve(Matrix_i* a, Matrix_i* b);
"# }}</code></pre>
<div>{ r#"Now we can implement our functions, pretty much the same way we did for our "# }<code>{ r#"Vector_i"# }</code>{ r#", which I will skip for brevity"# }</div>
<div>{ r#"So once we have all our helper functions we can start convolving matrices."# }</div>
<div>{ r#"This is going to be a lot simpler than the vector convolution because for the vector we did a biblically accurate convolution - where it is mathematically correct."# }</div>
<div>{ r#"For the matrix, we are essentially going to do the dumbed-down computer science version & just take a moving weighted sum of each element."# }</div>
<div>{ r#"I am also going to ignore all of the little edges and essentially treat the middle of our matrix b as the sole point of focus."# }</div>
<div>{ r#"And calculate an output for each element of a with b positioned such that its center is paired with the element of a in question."# }</div>
<div>{ r#"So, for example - If we were doing a biblically accurate matrix convolution, we would have to deal with cases like this:"# }</div>
<pre><code>{{ r#"
 _________
|         |
|         |
|    b    |
|        _|_______
|_______|_|       |
        |         |
        |    a    |
        |         |
        |_________|
"# }}</code></pre>
<div>{ r#"In the case where we are convolving b over a, above is the visual representation of the first step"# }</div>
<div>{ r#"where the overlap represents elements of each matrix that are 'paired up' for this iteration,"# }</div>
<div>{ r#"and therefore should be multiplied pair-wise and then summed to produce the output."# }</div>
<div>{ r#"If we were to do this, the resulting matrix would be larger than a..."# }</div>
<div>{ r#"How much larger? I'm not sure but we can figure that out now (since I don't feel like writing the function right now)"# }</div>
<div>{ r#"So we start with the matrices positioned as above."# }</div>
<div>{ r#"We perform an operation, shift, and repeat until we find a and b in the following configuration"# }</div>
<pre><code>{{ r#"
         _________
        |         |
        |         |
        |    b    |
        |_________|
        |_________|
        |         |
        |    a    |
        |         |
        |_________|
"# }}</code></pre>
<div>{ r#"In the case above b and a have the same width, but regardless of relative widths, we will have done "# }<code>{ r#"Matrix_i_cols(a)"# }</code>{ r#" operations and therefore have generated as many outputs."# }</div>
<div>{ r#"Then, we continue until just a single element of each is overlapping, like this"# }</div>
<pre><code>{{ r#"
                 _________
                |         |
                |         |
                |    b    |
         _______|_        |
        |       |_|_______|
        |         |
        |    a    |
        |         |
        |_________|
"# }}</code></pre>
<div>{ r#"So we've now done an additional "# }<code>{ r#"Matrix_i_cols(b) - 1"# }</code>{ r#" operations, and thus have generated as many outputs"# }</div>
<div>{ r#"Which puts us at "# }<code>{ r#"Matrix_i_cols(a) + Matrix_i_cols(b) - 1"# }</code>{ r#" "# }</div>
<div>{ r#"Flipping the whole thing on it's side we can see that the same holds true for the height"# }</div>
<div>{ r#"And so the biblically accurate output matrix has the following dimensions:"# }</div>
<div>{ r#"width: "# }<code>{ r#"Matrix_i_cols(a) + Matrix_i_cols(b) - 1"# }</code>{ r#""# }</div>
<div>{ r#"height: "# }<code>{ r#"Matrix_i_rows(a) + Matrix_i_rows(b) - 1"# }</code>{ r#""# }</div>
<div>{ r#"Anyway, now lets write the blasphemers version"# }</div>
<div>{ r#"For each element in a, we apply b as if its center is at the current element of a."# }</div>
<div>{ r#"Applying b involves pairing each element with an element from a by its relative position."# }</div>
<pre><code>{{ r#"
Matrix_i* Matrix_i_convolve(Matrix_i* a, Matrix_i* b) {
    size_t a_w = Matrix_i_cols(a);
    size_t a_h = Matrix_i_rows(a);

    size_t b_w = Matrix_i_cols(b);
    size_t b_h = Matrix_i_rows(b);

    Matrix_i* result = Matrix_i_new(a_w, a_h);

    for (size_t y=0; y<a_h; ++y) {
        for (size_t x=0; x<a_w; ++x) {
            uint8_t bytes[4] = {0, 0, 0, 0};
            int weighted_sum = 0;
            for (size_t i=0; i<b_h; ++i) {
                for (size_t j=0; j<b_w; ++j) {
                    int c = Matrix_i_get(b,i,j);
                    int a_x = ((x+j)-(b_w/2));
                    int a_y = ((y+i)-(b_h/2));

                    if ((a_x>-1) && (a_x<a_w) && (a_y>-1) && (a_y<a_h)) {
                        weighted_sum += Matrix_i_get(a,a_x,a_y) * c;
                    }
                }
            }
            Matrix_i_set(result,y,x, weighted_sum);
        }
    }
    return result;
}
"# }}</code></pre>
<div>{ r#"Here, we are calculating the index into a by adding the current index of a and b & subtracting half of the corresponding dimension of b."# }</div>
<div>{ r#"Now we can see if this thing is working..."# }</div>
</div>
<img src={"/build/20241113_convolution/images/9_matrix_convolution_initial.ong"}/>
<div>
</div><video autoplay=true width=500 loop=true>
<source src={"/build/20241113_convolution/images/10_matrix_output_initial.webm"} type="video/webm"/>
</video><div>
<div>{ r#"This doesn't really do anything for us though, what is the application for a discreet finite convolution of two integer matrices?"# }</div>
<div>{ r#"I'm sure someone smarter than me could find a use... but I'm not smarter than me yet"# }</div>
<div>{ r#"But what we can do is create new matrix types for unsigned 32-bit integers & floats, then convolve the floats over the ints"# }</div>
<div>{ r#"We can then encode image data in the "# }<code>{ r#"uint32_t"# }</code>{ r#" matrix and applying the convolution will change the image"# }</div>
<div>{ r#"So lets write that function:"# }</div>
<pre><code>{{ r#"
// cross-type convolution
Matrix_u32* Matrix_u32_f_convolve(Matrix_u32* a, Matrix_f* b) {
    size_t a_w = Matrix_u32_cols(a);
    size_t a_h = Matrix_u32_rows(a);

    size_t b_w = Matrix_f_cols(b);
    size_t b_h = Matrix_f_rows(b);

    Matrix_u32* result = Matrix_u32_new(a_w, a_h);

    for (size_t y=0; y<a_h; ++y) {
        for (size_t x=0; x<a_w; ++x) {
            uint32_t weighted_sum = 0;
            for (size_t i=0; i<b_h; ++i) {
                for (size_t j=0; j<b_w; ++j) {
                    float c = Matrix_f_get(b,i,j);
                    int a_x = ((x+j)-(b_w/2));
                    int a_y = ((y+i)-(b_h/2));

                    uint32_t pixel = Matrix_u32_get(a,x,y);
                    if ((a_x>-1) && (a_x<a_w) && (a_y>-1) && (a_y<a_h)) {
                        pixel = Matrix_u32_get(a,a_x,a_y);
                    }
                    // apply the mask to get the correct bytes
                    // shift them over to the right, multuply by the coefficient
                    // shift them back to where the were and add to the sum
                    weighted_sum += ((uint8_t)(((pixel&0x0000FF)>>8*0) * c))<<8*0;
                    weighted_sum += ((uint8_t)(((pixel&0x00FF00)>>8*1) * c))<<8*1;
                    weighted_sum += ((uint8_t)(((pixel&0xFF0000)>>8*2) * c))<<8*2;
                }
            }
            Matrix_u32_set(result,y,x, weighted_sum);
        }
    }
    return result;
}

"# }}</code></pre>
<div>{ r#"now this compiles, and we can use it in our main function..."# }</div>
<pre><code>{{ r#"
int main() {
    Matrix_u32* img = Matrix_u32_new(400, 400);
    Matrix_f* msk = Matrix_f_new(4, 4);
    tv_static(img);
    flat_blur_matrix(msk);
    save_img_as_ppm(img, "original.ppm");
    Matrix_u32* blr = Matrix_u32_f_convolve(img, msk);
    save_img_as_ppm(blr, "blurred.ppm");
    return 0;
}
"# }}</code></pre>
<div>{ r#"The first thing we do above is create a 400x400 matrix to represent out image."# }</div>
<div>{ r#"Then, we create a 4x4 mask that will be the box which we move over our image to blur it."# }</div>
<div>{ r#"Then we fill the image with either black or white pixels by passing its pointer to the "# }<code>{ r#"tv_static"# }</code>{ r#" function, which looks like this:"# }</div>
<pre><code>{{ r#"
void tv_static(Matrix_u32* img) {
    size_t w = Matrix_u32_cols(img);
    size_t h = Matrix_u32_rows(img);

    for (int y=0; y<h; ++y) {
        for (int x=0; x<w; ++x) {
            uint32_t pixel = 0xFF000000;
            if ((int)((rand()/(float)RAND_MAX) + 0.5)) {
                pixel = 0xFFFFFF;
            }
            Matrix_u32_set(img,x,y,pixel);
        }
    }
}
"# }}</code></pre>
<div>{ r#"Then we can fill up our msk matrix with values that sum to 1.0f with a function like this:"# }</div>
<pre><code>{{ r#"
void flat_blur_matrix(Matrix_f* msk) {
    size_t msk_w = Matrix_f_cols(msk);
    size_t msk_h = Matrix_f_rows(msk);
    for (int y=0; y<msk_h; ++y) {
        for (int x=0; x<msk_w; ++x) {
            Matrix_f_set(msk,y,x,1.0/(msk_w*msk_h));
        }
    }
}
"# }}</code></pre>
<div>{ r#"Then, I use this "# }<code>{ r#"save_image_as_ppm"# }</code>{ r#" to output the data into an image file with the given name, PPM being the choice of format because it's header is extremely simple:"# }</div>
<pre><code>{{ r#"
P6
WIDTH HEIGHT 255
RGB_IMAGE_BYTES
"# }}</code></pre>
<div>{ r#"And this is the function used to generate that file:"# }</div>
<pre><code>{{ r#"
void save_img_as_ppm(Matrix_u32* img, const char *file_path) {
    FILE* f = fopen(file_path, "wb");
    if (f == NULL) {
        fprintf(stderr, "ERROR: could not write into file %s: %s\n", file_path, strerror(errno));
        exit(1);
    }
    size_t img_w = Matrix_u32_cols(img);
    size_t img_h = Matrix_u32_rows(img);
    fprintf(f, "P6\n%d %d 255\n", img_w, img_h);
    for (size_t y=0; y<img_h; ++y) {
        for (size_t x=0; x<img_w; ++x) {
            uint32_t pixel = Matrix_u32_get(img,x,y);
            uint8_t bytes[3] = {
                (pixel&0x0000FF)>>8*0,
                (pixel&0x00FF00)>>8*1,
                (pixel&0xFF0000)>>8*2
            };
            fwrite(bytes, sizeof(bytes), 1, f);
            assert(!ferror(f));
        }
    }
    fclose(f);
}
"# }}</code></pre>
<div>{ r#"Then we apply our blur to the matrix, generating a new image data matrix, and call the same function on it to save it as a PPM image file."# }</div>
<div>{ r#"And just like that, we have our original.ppm:"# }</div>
</div>
<img src={"/build/20241113_convolution/images/11_original.png"}/>
<div>
<div>{ r#"And our blurred.ppm image:"# }</div>
</div>
<img src={"/build/20241113_convolution/images/12_blurred.png"}/>
<div>
<div>{ r#""# }</div>
<div>{ r#""# }</div>
</div></span>});	post_map.insert(String::from("20241106_image_resizing"), html! {<span markdown="block" style="white-space: pre-wrap"><div markdown="span">
<h1>{ r#"Image resizing blog"# }</h1>
<div>{ r#""# }</div>
<div>{ r#"Start by creating cargo project"# }</div>
</div>
<img src={"/build/20241106_image_resizing/images/1_aw_nutz.png"}/>
<div>
<div>{ r#""# }</div>
<div>{ r#"copy shell.nix from another project & start the shell"# }</div>
<div>{ r#""# }</div>
<div>{ r#"where is my fish?"# }</div>
</div>
<img src={"/build/20241106_image_resizing/images/2_where_my_fish.png"}/>
<div>
<div>{ r#""# }</div>
<div>{ r#"exit that shell"# }</div>
</div><video autoplay=true width=500 loop=true>
<source src={"/build/20241106_image_resizing/images/3_exit_dirty_bash.webm"} type="video/webm"/>
</video><div>
<div>{ r#""# }</div>
<div>{ r#"look up youtube video"# }</div>
<div>{ r#" - https://www.youtube.com/watch?v=tv9s4jhdUpU"# }</div>
<div>{ r#""# }</div>
<div>{ r#"come back in a few days and start writing the resizer again"# }</div>
<div>{ r#"So far, this is what we have in main.rs:"# }</div>
<pre><code>{{ r#"
use axum::{
    routing::get,
    body::Bytes,
    http::StatusCode,
    extract::Query,
    Router,
    debug_handler,
};
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize)]
struct Task {
    width: Option<u32>,
    height: Option<u32>,
    url: String,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    // tracing_subscriber::fmt()
        // .compact().with_file(true)
        // .with_line_number(true)
        // .with_thread_ids(true)
        // .with_target(true).finish();
    let sub = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(sub).unwrap();


    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/resize", get(resize));
    info!("starting");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
#[debug_handler]
async fn resize(task: Query<Task>) -> (StatusCode, Bytes) {
    // we need to fetch the image here
    info!(
        "Received request: url: {:?} w: {:?} h: {:?}",
        task.url, task.width, task.height
    );
    (StatusCode::OK, task.url.as_bytes().to_vec().into())
}
"# }}</code></pre>
<div>{ r#"when we run it, we get this in the logs: "# }</div>
</div>
<img src={"/build/20241106_image_resizing/images/4_create_project.png"}/>
<div>
<div>{ r#"and we get this back when we send a request:"# }</div>
</div>
<img src={"/build/20241106_image_resizing/images/5_first_response.png"}/>
<div>
<div>{ r#""# }</div>
<div>{ r#"Now, we'll write a module that fetches images for us and returns them in a DynamicImage"# }</div>
<div>{ r#"First, we'll restructure what we have & create files for our image fetching module"# }</div>
<div>{ r#"It'll live with all of our http-client-like things, so we'll name the module http"# }</div>
<div>{ r#"We'll also introduce an error module to make using the "# }<code>{ r#"?"# }</code>{ r#" easier."# }</div>
<div>{ r#"And just like that our structure looks like this:"# }</div>
</div>
<img src={"/build/20241106_image_resizing/images/6_new_file_structure.png"}/>
<div>
<div>{ r#"And our "# }<code>{ r#"src/error.rs"# }</code>{ r#" looks like this:"# }</div>
<pre><code>{{ r#"
// errors for the service

use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
}

"# }}</code></pre>
<div>{ r#"Note that we also added the "# }<code>{ r#"derive_more"# }</code>{ r#" crate. ("# }<code>{ r#"cargo add derive_more --features full"# }</code>{ r#")"# }</div>
<div>{ r#"This will come into play later when converting from external errors to our service errors."# }</div>
<div>{ r#""# }</div>
<div>{ r#"Now, we'll start converting all of our "# }<code>{ r#"unwrap()"# }</code>{ r#"s into "# }<code>{ r#"?"# }</code>{ r#" operators."# }</div>
<div>{ r#"Starting in lib.rs, we make "# }<code>{ r#"run"# }</code>{ r#" return a "# }<code>{ r#"Result<()>"# }</code>{ r#" and..."# }</div>
<pre><code>{{ r#"
error[E0277]: `?` couldn't convert the error to `error::Error`
  --> src/lib.rs:50:37
   |
50 |     axum::serve(listener, app).await?;
   |                                     ^ the trait `From<std::io::Error>` is not implemented for `error::Error`, which is required by `std::result::Result<(), error::Error>: FromResidual<std::result::Result<Infallible, std::io::Error>>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: the trait `FromResidual<std::result::Result<Infallible, E>>` is implemented for `std::result::Result<T, F>`
   = note: required for `std::result::Result<(), error::Error>` to implement `FromResidual<std::result::Result<Infallible, std::io::Error>>`
"# }}</code></pre>
<div>{ r#"now it yells at us to make our error implement "# }<code>{ r#"From"# }</code>{ r#" for "# }<code>{ r#"std::io::Error"# }</code>{ r#"."# }</div>
<div>{ r#"This should be easy with "# }<code>{ r#"derive_more"# }</code>{ r#"."# }</div>
<pre><code>{{ r#"
#[derive(From, Debug)]
pub enum Error {
    // External
    #[from]
    TracingGlobalDefault(tracing::subscriber::SetGlobalDefaultError),
    #[from]
    ServerBinding(std::io::Error),
}
"# }}</code></pre>
<div>{ r#"And no more errors! We'll need to add more to this enum while implementing our image fetcher..."# }</div>
<div>{ r#"So we'll do that now"# }</div>
<div>{ r#"We want to be caching as much as possible in order to speed this thing up because image processing pretty slow."# }</div>
<div>{ r#"In order to fetch images faster, we should actually send two requests - "# }</div>
<div>{ r#"    1. Grab the data from the provided image url"# }</div>
<div>{ r#"    2. Grab the data from a local image store if possible"# }</div>
<div>{ r#"Typically this would be done in order, but we can use tokio to just fire off two requests and take whichever comes back successfully first."# }</div>
<div>{ r#"I'm not certain this is actually a performance improvement for these reasons:"# }</div>
<div>{ r#"    - Requesting a local image that doesn't exist will 404 very quickly anyway"# }</div>
<div>{ r#"    - Always sending off two requests rather than sometimes only sending off one could be detrimental under heavy load"# }</div>
<div>{ r#"I'm going to do it anyway for these reasons:"# }</div>
<div>{ r#"    - Learning experience"# }</div>
<div>{ r#"    - Sometimes you have to get fancy and confusing to discover simplicity"# }</div>
<div>{ r#"    - Gives me an opportunity to find out which is faster later"# }</div>
<div>{ r#"So, I'll be creating two different implementations of our image fetching function"# }</div>
<div>{ r#""# }<code>{ r#"serial_fetch"# }</code>{ r#""# }</div>
<div>{ r#""# }<code>{ r#"parallel_fetch"# }</code>{ r#""# }</div>
<div>{ r#"Lets get into it..."# }</div>
<div>{ r#""# }</div>
<div>{ r#"In order to write a simple http client we'll use hyper, since axum already uses it"# }</div>
<div>{ r#"And I'll be starting from this example: "# }<code>{ r#"https://hyper.rs/guides/1/client/basic/"# }</code>{ r#""# }</div>
<div>{ r#"Making a single request to begin with, we have something like this:"# }</div>
<pre><code>{{ r#"
#[tracing::instrument]
pub async fn serial_fetch(url: String) -> Result<DynamicImage> {
    // parse our url
    let url = url.parse::<hyper::Uri>()?;

    // get the host and the port
    let host = url.host().unwrap();
    let port = url.port_u16().unwrap_or(80);

    let address = format!("{}:{}", host, port);

    // Open TCP connection to the remote host
    let stream = TcpStream::connect(address).await?;

    let io = TokioIo::new(stream);

    // create the Hyper Client
    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

    // spawn a task to poll the connection, driving the http state
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            error!("Connection failed: {:?}", err);
        };
    });

    // the authority of our url will be the host
    let authority = url.authority().unwrap().clone();

    // create an http request with an empty body & host header
    let req = Request::builder()
        .uri(url)
        .header(hyper::header::HOST, authority.as_str())
        .body(Empty::<Bytes>::new())?;

    // await the response
    let mut res = sender.send_request(req).await?;

    let mut image_bytes = Vec::<u8>::new();

    while let Some(next) = res.frame().await {
        let frame = next?;
        if let Some(chunk) = frame.data_ref() {
            image_bytes.extend(chunk);
        }

    }

    let mut img = ImageReader::new(std::io::Cursor::new(image_bytes));
    img.set_format(ImageFormat::Png);
    let img = img.decode()?;

    Ok(img)

}
"# }}</code></pre>
<div>{ r#"Some TODO's for this thing:"# }</div>
<div>{ r#"    - abstract away the 'client' construct"# }</div>
<div>{ r#"    - parsing the bytes into an image should be separate"# }</div>
<div>{ r#"        - this is going to get more complicated with more formats"# }</div>
<div>{ r#""# }</div>
<div>{ r#"For now, we can start using this thing by updating our handler in lib.rs like this:"# }</div>
<pre><code>{{ r#"
async fn resize(task: Query<Task>) -> impl IntoResponse {
    // we need to fetch the image here
    info!(
        "Received request: url: {:?} w: {:?} h: {:?}",
        task.url, task.width, task.height
    );
    match image_fetcher::serial_fetch(task.url.clone()).await {
        Ok(img) => {
            let mut bytes = Vec::<u8>::new();
            img.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png).unwrap();
            return (
                StatusCode::OK,
                AppendHeaders([
                    ("Content-Type", "image/png")
                ]),
                bytes.into()
            );
        },
        Err(err) => {
            error!("Could not get the image {:?}", err);
            return (
                StatusCode::NOT_FOUND,
                AppendHeaders([
                    ("Content-Type", "image/png"),
                ]),
                Bytes::new()
            );
        },
    }
}
"# }}</code></pre>
<div>{ r#"For convenience, we write a little test script "# }<code>{ r#"test_curl"# }</code>{ r#":"# }</div>
<pre><code>{{ r#"
#! /bin/sh

curl \
    "localhost:3000/resize?url=http%3A%2F%2Fstatic.wikia.nocookie.net%2Fadventuretimewithfinnandjake%2Fimages%2F9%2F9e%2FPeppermint_Butler.png" \
    --verbose \
    --output out.png
"# }}</code></pre>
<div>{ r#"and then we can "# }<code>{ r#"cargo r"# }</code>{ r#" and "# }<code>{ r#"./test_curl"# }</code>{ r#" to see if it works"# }</div>
</div>
<img src={"/build/20241106_image_resizing/images/7_test_curl_logs.png"}/>
<div>
</div>
<img src={"/build/20241106_image_resizing/images/8_test_curl_output.png"}/>
<div>
<div>{ r#"Great!"# }</div>
<div>{ r#"Now to actually resize the image..."# }</div>
<div>{ r#"This functionality should definitely be put into another module - we're no longer dealing with http requests."# }</div>
<div>{ r#"So we create a new module img which will contain all of our image parsing and manipulation stuff"# }</div>
</div>
<img src={"/build/20241106_image_resizing/images/9_new_img_module.png"}/>
<div>
<div>{ r#"And insize "# }<code>{ r#"img/resizer.rs"# }</code>{ r#" we can add something like this to get it resizing images..."# }</div>
<pre><code>{{ r#"
// image manipulation functionality

use image::DynamicImage;

pub async fn resize(image: &mut DynamicImage, width: u32, height: u32) -> DynamicImage {
    image.resize_exact(width, height, image::imageops::FilterType::Lanczos3)
}
"# }}</code></pre>
<div>{ r#"And then update our "# }<code>{ r#"resize"# }</code>{ r#" handler in lib.rs to call to a function that uses our new "# }<code>{ r#"resizer::resize"# }</code>{ r#" function:"# }</div>
<pre><code>{{ r#"
async fn handle_resize(task: Task) -> Result<Vec<u8>> {
    match image_fetcher::serial_fetch(task.url.clone()).await {
        Ok(img) => {
            let img = resizer::resize(
                &mut img.clone(), 
                task.width.expect("NO WIDTH FOUND!"),
                task.height.expect("NO HEIGHT FOUND!")
            ).await;
            let mut bytes = Vec::<u8>::new();
            img.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png).unwrap();
            return Ok(bytes);
        },
        Err(err) => {
            error!("Could not get the image {:?}", err);
            return Err(err)
        },
    }
}

#[debug_handler]
async fn resize(Query(task): Query<Task>) -> impl IntoResponse {
    // we need to fetch the image here
    info!(
        "Received request: url: {:?} w: {:?} h: {:?}",
        task.url, task.width, task.height
    );
    match handle_resize(task.into()).await {
        Ok(bytes) => {
            (
                StatusCode::OK,
                AppendHeaders([
                    ("Content-Type", "image/png")
                ]),
                bytes.into()
            )
        },
        Err(_err) => {
            (
                StatusCode::NOT_FOUND,
                AppendHeaders([
                    ("Content-Type", "image/png"),
                ]),
                Bytes::new()
            )
        },
    }
}
"# }}</code></pre>
<div>{ r#"And now we can update our "# }<code>{ r#"test_curl"# }</code>{ r#" script to include a "# }<code>{ r#"width"# }</code>{ r#" and "# }<code>{ r#"height"# }</code>{ r#" parameters"# }</div>
<div>{ r#"And just like that we have a slightly wider peppermint butler"# }</div>
</div>
<img src={"/build/20241106_image_resizing/images/10_wide_pep_but.png"}/>
<div>
<div>{ r#""# }</div>
</div></span>});
        Self {
            posts: post_map,
        }
    }
}