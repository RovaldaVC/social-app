from fastapi import APIRouter, Depends
from database.database import get_db
from sqlalchemy.orm import Session
from repository import authentication
from fastapi.security import OAuth2PasswordRequestForm
from database import schema

router = APIRouter(
    prefix="/auth",
    tags=["authentication"]
)

@router.post("/")
def authenticate(request: OAuth2PasswordRequestForm = Depends(), db: Session = Depends(get_db)):
    request_data = schema.Auth(username=request.username, password=request.password)
    return authentication.auth(request_data, db)
