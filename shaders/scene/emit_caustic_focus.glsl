#include geom/mod



float sampleWavelength(RAND_PARAM, float t) {
//        return RAND_FLOAT * 300.0 + 400.0;
    return t * 300.0 + 400.0;
//    return t * 200.0 + 500.0;
}

EdgeSample sampleLightsSpaced(RAND_PARAM, float t) {
//    vec2 n = vec2(1, 0);
        vec2 n = normalize(vec2(1.0, 0.8));
    if (RAND_FLOAT < -0.5) {
        n = RAND_HEMI(n);
    }
    float thick = 0.0;
    return EdgeSample(vec2(-100.0, -210.0 + screenSize.y * 0.5 + (t - 0.5) * thick), n);
}

float varyParam(float s, float o, float a, float b) {
    return a + (b - a) * (0.5+0.5*cos(systemTime * s + o));
}

float getEta(float wl) {
//    return 1.0 / 1.0 * 400.0 / wl;
    return 1.0 / mix(1.5, 3.0, (wl - 400.0)/ 700.0);
}

#define H_SCENE_EPS 1E10
HitRes hitScene(Ray r) {
    HitRes hr;
    hr.t = H_SCENE_EPS;

    hitCircle(r, Circle(vec2(600.0 + varyParam(0.7, -1.0, -80.0, 40.0), 400.0 + varyParam(0.5, 0.0, -50.0, 50.0)), 150.0, M_REFR), hr);

    float x = 200.0;
    hitLine(r, Line(vec2(x-100.0, 300.0), vec2(x+100.0, 300.0), M_REFR), hr);
    hitLine(r, Line(vec2(x, 500.0), vec2(x+100.0, 300.0), M_REFR), hr);
    hitLine(r, Line(vec2(x, 500.0), vec2(x-100.0, 300.0), M_REFR), hr);

//    hitArc(r, Arc(vec2(800.0, 400.0), 500.0, -90.0, 90.0, M_REFL), hr);
//    hitArc(r, Arc(vec2(800.0, 400.0), 500.0, varyParam(0.8, 1.0, -60.0, -120.0), varyParam(0.6, 2.0, 60.0, 120.0), M_REFL), hr);
    hitArc(r, Arc(vec2(800.0, 400.0), 500.0, -120.0, 120.0, M_REFL), hr);

//    hitLine(r, Line(vec2(0, 0), vec2(screenSize.x, 0), M_REFL), hr);
//    hitLine(r, Line(vec2(screenSize.x, 0), screenSize, M_REFL), hr);
//    hitLine(r, Line(screenSize, vec2(0, screenSize.y), M_REFL), hr);
//    hitLine(r, Line(vec2(0, screenSize.y), vec2(0, 0), M_REFL), hr);

    hr.hit = hr.t < H_SCENE_EPS;
    if (hr.hit) {
        hr.n = normalize(hr.n);
        if (dot(hr.n, r.d) > 0.0) {
            hr.n *= -1.0;
        }
    }
    return hr;
}