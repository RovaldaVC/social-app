from sqlalchemy.orm import Session
from database import schema, models
from security.hashing import Hash
from fastapi import HTTPException, status

def create_user(request: schema.CreateUser, db: Session):
    new_user = models.usersDB(
        name=request.name,
        family=request.family,
        username=request.username,
        password=Hash.bcrypt(request.password)
    )
    db.add(new_user)
    db.commit()
    db.refresh(new_user)
    return new_user


def find_user(id: int, db: Session):
    user = db.query(models.usersDB).filter(models.usersDB.id == id).first()
    if not user:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail=f"User with id {id} not found."
        )
    return user