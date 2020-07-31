#include "IrrklangIntegration.h"
#include <iostream>
#include <Irrklang.h>

class Manager {
    void Initialize();
    irrklang::vec3df toVec3df(const Vector& position);
    void setListPos(const Vector& position, const Vector& lookDirection, const Vector& vec, const Vector& upDir)
};