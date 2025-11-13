from pydantic import BaseModel

class CreateUser(BaseModel):
    name: str
    family: str
    username: str
    password: str


class ShowUser(BaseModel):
    name: str
    family: str

    class Config:
        orm_mode = True


class Auth(BaseModel):
    username: str
    password: str


class Token(BaseModel):
    access_token: str
    token_type: str
